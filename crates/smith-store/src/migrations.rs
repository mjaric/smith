//! Forward-only schema migrations (REQ-PERS-009, REQ-PERS-010).
//!
//! Migrations are append-only: [`REGISTRY`] is extended at the tail only,
//! versions are dense and 1-based, and there are no down-migrations — a
//! broken migration is a bug to fix forward, not to roll back. On open, the
//! runner reads `smith_meta.schema_version`, applies every pending migration
//! in a single transaction, and bumps the version. A failed migration rolls
//! the transaction back and aborts open with a clear error.

use std::path::Path;

use rusqlite::{Connection, OptionalExtension};

use crate::error::Error;

/// One forward migration: a version, a name, and the SQL that takes the
/// schema from `version - 1` to `version`.
///
/// There is deliberately no down/revert SQL (REQ-PERS-010).
pub struct Migration {
    /// Target schema version (1-based, dense).
    pub version: u32,
    /// Human-readable name used in error messages.
    pub name: &'static str,
    /// The forward DDL.
    pub sql: &'static str,
}

/// Every shipped migration, in version order. Append-only (REQ-PERS-010).
pub const REGISTRY: &[Migration] = &[Migration {
    version: 1,
    name: "initial schema",
    sql: include_str!("migrations/v001_initial_schema.sql"),
}];

/// The schema version a fully migrated file is at: the version of the last
/// registered migration (0 when the registry is empty).
#[must_use]
pub fn current_schema_version() -> u32 {
    match REGISTRY.last() {
        Some(migration) => migration.version,
        None => 0,
    }
}

/// Create `smith_meta` and set `schema_version` to 0 if the table does not
/// exist yet (brand-new or pre-migration files).
pub(crate) fn bootstrap_meta(conn: &Connection) -> Result<(), rusqlite::Error> {
    let has_meta: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'smith_meta')",
        [],
        |row| row.get(0),
    )?;
    if !has_meta {
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS smith_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL); \
             INSERT INTO smith_meta (key, value) VALUES ('schema_version', '0');",
        )?;
        tx.commit()?;
    }
    Ok(())
}

/// Read `smith_meta.schema_version`; a missing row means version 0.
///
/// # Errors
///
/// Returns [`Error::CorruptSchemaVersion`] when the value is not an integer.
pub(crate) fn read_schema_version(conn: &Connection, path: &Path) -> Result<u32, Error> {
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM smith_meta WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|detail| Error::from_sqlite(path, detail))?;
    match value {
        None => Ok(0),
        Some(raw) => raw.parse::<u32>().map_err(|_| Error::CorruptSchemaVersion {
            path: path.to_path_buf(),
            detail: format!("'{raw}' is not an integer"),
        }),
    }
}

/// Apply every migration newer than `version` in one transaction, then bump
/// `smith_meta.schema_version` (REQ-PERS-009).
///
/// # Errors
///
/// Returns [`Error::MigrationFailed`] when any migration's SQL fails — the
/// transaction is rolled back and the file is left untouched;
/// [`Error::CorruptSchemaVersion`] when the recorded version is ahead of the
/// registry.
pub(crate) fn run_migrations(conn: &Connection, path: &Path, version: u32) -> Result<u32, Error> {
    let latest = current_schema_version();
    if version > latest {
        return Err(Error::CorruptSchemaVersion {
            path: path.to_path_buf(),
            detail: format!(
                "schema_version {version} is newer than the latest migration ({latest}); \
                 this file was written by a newer Smith"
            ),
        });
    }
    let pending = &REGISTRY[version as usize..];
    if pending.is_empty() {
        return Ok(version);
    }

    let tx = conn
        .unchecked_transaction()
        .map_err(|detail| Error::from_sqlite(path, detail))?;
    for migration in pending {
        tx.execute_batch(migration.sql)
            .map_err(|detail| Error::MigrationFailed {
                version: migration.version,
                name: migration.name,
                path: path.to_path_buf(),
                detail: detail.to_string(),
            })?;
    }
    tx.execute(
        "UPDATE smith_meta SET value = ?1 WHERE key = 'schema_version'",
        [latest],
    )
    .map_err(|detail| Error::from_sqlite(path, detail))?;
    tx.commit()
        .map_err(|detail| Error::from_sqlite(path, detail))?;
    Ok(latest)
}
