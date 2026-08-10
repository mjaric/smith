//! `SQLite` persistence for Smith: the `.smith` file format, forward-only
//! schema migrations, and validated JSON data blobs.
//!
//! A `.smith` file is a single `SQLite` database (`REQ-PERS-017`), inspectable
//! by any sqlite3 tool (`REQ-PERS-018`), identified by a Smith-specific
//! `PRAGMA application_id` magic (`REQ-PERS-019`), opened in WAL mode with
//! `synchronous=NORMAL` and foreign keys on (`REQ-PERS-011`), and backed up
//! on every successful open with rotation to the last three versions
//! (`REQ-PERS-020`). The schema evolves through the append-only migrations in
//! [`migrations`] (`REQ-PERS-009`, `REQ-PERS-010`); JSON `data` blobs are
//! serde round-trip helpers in [`data`] (`REQ-PERS-005`); ownership-closure-
//! table maintenance (create/reparent reads, atomic with the owning edge) is
//! in [`closure`] (`REQ-PERS-007`, `REQ-PERS-008`).
//!
//! Non-goals of this crate: the model API, FTS5 search queries, and diagrams
//! CRUD.

pub mod closure;
pub mod data;
pub mod error;
pub mod migrations;

use std::fmt;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

pub use data::{deserialize_data, serialize_data, DataError};
pub use error::Error;
pub use migrations::{current_schema_version, Migration, REGISTRY};

/// Smith's `PRAGMA application_id` magic (`REQ-PERS-019`): the ASCII bytes
/// of `"SMTH"` read big-endian (0x534D5448). Written to byte 68 of the
/// `SQLite` header and verified on every open.
pub const APPLICATION_ID: i64 = 0x534D_5448;

/// `SQLite` file header magic (first 16 bytes of every `SQLite` database).
const SQLITE_HEADER: &[u8; 16] = b"SQLite format 3\0";

/// Maximum number of rotated `.bak.N` backups kept (REQ-PERS-020).
const BACKUP_COUNT: usize = 3;

/// A `.smith` project file: opened `SQLite` connection plus derived metadata.
///
/// Dropping the store closes the connection after a best-effort `TRUNCATE`
/// WAL checkpoint, so committed data lives in the single main file
/// (`REQ-PERS-017`). Use [`Store::close`] to observe checkpoint errors.
pub struct Store {
    conn: Connection,
    path: PathBuf,
    schema_version: u32,
}

impl fmt::Debug for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Store")
            .field("path", &self.path)
            .field("schema_version", &self.schema_version)
            .finish_non_exhaustive()
    }
}

impl Store {
    /// The file this store is backed by.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The schema version this file was migrated to on open.
    #[must_use]
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// The live `SQLite` connection (single writer; see REQ-PERS-012).
    #[must_use]
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Clean close: fold the WAL into the main file via a `TRUNCATE`
    /// checkpoint (`REQ-PERS-017`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Sqlite`] when the checkpoint fails.
    pub fn close(self) -> Result<(), Error> {
        self.checkpoint()
    }

    fn checkpoint(&self) -> Result<(), Error> {
        self.conn
            .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
            .map_err(|detail| Error::Sqlite {
                path: self.path.clone(),
                detail,
            })
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        let _ = self.checkpoint();
    }
}

/// Open (or create) a `.smith` file (`REQ-PERS-017`).
///
/// Steps:
/// 1. Verify the `SQLite` header and the Smith `application_id` magic,
///    rejecting non-Smith files with a clear error (`REQ-PERS-019`); new
///    files get the magic set.
/// 2. Configure `journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`
///    (`REQ-PERS-011`).
/// 3. Snapshot the previous version (pre-migration state) to a staging file
///    (`REQ-PERS-020`).
/// 4. Bootstrap `smith_meta`, read `schema_version`, and run pending
///    migrations in one transaction (`REQ-PERS-009`).
/// 5. On success, rotate the staged snapshot into the `.bak.N` series,
///    keeping the last 3 (`REQ-PERS-020`).
///
/// Backups capture the pre-open state (the previous version): the snapshot is
/// taken via `VACUUM INTO` before any migration runs, and is only rotated
/// into place after the open succeeds, so a failed open never rotates
/// backups and never rotates a corrupt file in.
///
/// # Errors
///
/// - [`Error::NotASqliteDatabase`] — the file is not `SQLite` at all.
/// - [`Error::NotASmithFile`] — a `SQLite` file without the Smith magic.
/// - [`Error::WalUnavailable`] — WAL journal mode could not be enabled.
/// - [`Error::MigrationFailed`] / [`Error::CorruptSchemaVersion`] — the
///   schema could not be brought to the current version; open aborts and the
///   file is untouched.
/// - [`Error::BackupFailed`] / [`Error::Io`] — backup rotation or IO failed.
pub fn open(path: &Path) -> Result<Store, Error> {
    let path = path.to_path_buf();
    let existed_before = is_smith_file(&path)?;

    let conn = Connection::open(&path).map_err(|detail| Error::from_sqlite(&path, detail))?;
    if !existed_before {
        conn.execute_batch(&format!("PRAGMA application_id = {APPLICATION_ID};"))
            .map_err(|detail| Error::from_sqlite(&path, detail))?;
    }
    configure_pragmas(&conn, &path)?;

    let staging = path.with_extension("smith.bak.tmp");
    if existed_before {
        if let Err(err) = snapshot_previous_version(&conn, &path, &staging) {
            let _ = fs::remove_file(&staging);
            return Err(err);
        }
    }

    let migration_result = migrations::bootstrap_meta(&conn)
        .map_err(|detail| Error::from_sqlite(&path, detail))
        .and_then(|()| migrations::read_schema_version(&conn, &path))
        .and_then(|version| migrations::run_migrations(&conn, &path, version));
    let schema_version = match migration_result {
        Ok(schema_version) => schema_version,
        Err(err) => {
            let _ = fs::remove_file(&staging);
            return Err(err);
        }
    };

    if existed_before {
        finalize_backup(&path, &staging)?;
    }

    Ok(Store {
        conn,
        path,
        schema_version,
    })
}

fn configure_pragmas(conn: &Connection, path: &Path) -> Result<(), Error> {
    // journal_mode persists in the database header; synchronous/foreign_keys
    // are per-connection and set on every open (REQ-PERS-011).
    conn.execute_batch(
        "PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA foreign_keys=ON;",
    )
    .map_err(|detail| Error::from_sqlite(path, detail))?;
    let mode: String = conn
        .pragma_query_value(None, "journal_mode", |row| row.get(0))
        .map_err(|detail| Error::from_sqlite(path, detail))?;
    if mode != "wal" {
        return Err(Error::WalUnavailable {
            path: path.to_path_buf(),
            mode,
        });
    }
    Ok(())
}

/// Whether `path` is an existing Smith file: `SQLite` header + Smith magic.
///
/// Returns `false` for a missing or zero-byte file (open creates those).
///
/// # Errors
///
/// [`Error::NotASqliteDatabase`] for an existing file without the `SQLite`
/// header; [`Error::NotASmithFile`] for a `SQLite` file without the Smith
/// magic; [`Error::Io`] on read failures.
fn is_smith_file(path: &Path) -> Result<bool, Error> {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(detail) => {
            return Err(Error::Io {
                path: path.to_path_buf(),
                detail,
            })
        }
    };
    if metadata.len() == 0 {
        // SQLite treats a zero-byte file as a new database; so do we.
        return Ok(false);
    }
    if metadata.len() < 72 {
        // Shorter than a SQLite header (magic + application_id field).
        return Err(Error::NotASqliteDatabase {
            path: path.to_path_buf(),
        });
    }

    let mut file = fs::File::open(path).map_err(io_err(path))?;
    let mut header = [0u8; 72];
    file.read_exact(&mut header).map_err(io_err(path))?;

    if &header[0..16] != SQLITE_HEADER {
        return Err(Error::NotASqliteDatabase {
            path: path.to_path_buf(),
        });
    }
    let app_id = i32::from_be_bytes([header[68], header[69], header[70], header[71]]);
    if i64::from(app_id) != APPLICATION_ID {
        return Err(Error::NotASmithFile {
            path: path.to_path_buf(),
        });
    }
    Ok(true)
}

/// Snapshot the current database state via `VACUUM INTO`: a consistent,
/// checkpointed, single-file copy regardless of WAL state.
fn snapshot_previous_version(conn: &Connection, path: &Path, staging: &Path) -> Result<(), Error> {
    let _ = fs::remove_file(staging);
    let staging_str = staging.to_string_lossy();
    conn.execute_batch(&format!("VACUUM INTO '{staging_str}';"))
        .map_err(|detail| Error::BackupFailed {
            path: path.to_path_buf(),
            detail: std::io::Error::other(detail.to_string()),
        })?;
    Ok(())
}

/// Rotate the staged snapshot into the `.bak.N` series: it becomes `.bak.1`,
/// existing backups shift one up, and anything beyond `.bak.3` is dropped
/// (REQ-PERS-020).
///
/// # Errors
///
/// [`Error::BackupFailed`] when a rename, remove, or copy fails.
fn finalize_backup(path: &Path, staging: &Path) -> Result<(), Error> {
    let backup = |n: usize| path.with_extension(format!("smith.bak.{n}"));
    let fail = |detail| Error::BackupFailed {
        path: path.to_path_buf(),
        detail,
    };

    if backup(BACKUP_COUNT).exists() {
        fs::remove_file(backup(BACKUP_COUNT)).map_err(fail)?;
    }
    for slot in (1..BACKUP_COUNT).rev() {
        if backup(slot).exists() {
            fs::rename(backup(slot), backup(slot + 1)).map_err(fail)?;
        }
    }
    fs::rename(staging, backup(1)).map_err(fail)?;
    Ok(())
}

fn io_err(path: &Path) -> impl FnOnce(std::io::Error) -> Error + use<> {
    let path = path.to_path_buf();
    move |detail| Error::Io { path, detail }
}
