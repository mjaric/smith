//! Error types for opening and operating on `.smith` files.

use std::io;
use std::path::{Path, PathBuf};

/// Every error `smith-store` can raise.
///
/// Variants carry the offending path and enough detail for a user-facing
/// message; no variant panics.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The file exists but is not a `SQLite` database.
    #[error("{path} is not a SQLite database; Smith only opens SQLite-backed .smith files")]
    NotASqliteDatabase {
        /// The rejected file.
        path: PathBuf,
    },

    /// The file is a `SQLite` database but not a Smith file
    /// (`application_id` magic mismatch, REQ-PERS-019).
    #[error(
        "{path} is not a Smith file: its SQLite application_id is not the Smith magic; \
         refusing to open a non-Smith database"
    )]
    NotASmithFile {
        /// The rejected file.
        path: PathBuf,
    },

    /// WAL journal mode could not be enabled (REQ-PERS-011).
    #[error("{path} could not be switched to WAL journal mode (got '{mode}')")]
    WalUnavailable {
        /// The affected file.
        path: PathBuf,
        /// The journal mode `SQLite` actually reported.
        mode: String,
    },

    /// `smith_meta.schema_version` is present but unusable.
    #[error("{path} has a corrupt smith_meta.schema_version: {detail}")]
    CorruptSchemaVersion {
        /// The affected file.
        path: PathBuf,
        /// Why the version is unusable.
        detail: String,
    },

    /// A migration failed; open aborted and the file was left untouched
    /// (REQ-PERS-009).
    #[error("migration V{version:03} ({name}) failed on {path}: {detail}")]
    MigrationFailed {
        /// The version of the failed migration.
        version: u32,
        /// The migration's name.
        name: &'static str,
        /// The affected file.
        path: PathBuf,
        /// The underlying `SQLite` error.
        detail: String,
    },

    /// Backup rotation failed (REQ-PERS-020).
    #[error("backup rotation failed for {path}: {detail}")]
    BackupFailed {
        /// The affected file.
        path: PathBuf,
        /// The underlying IO error.
        #[source]
        detail: io::Error,
    },

    /// IO error while inspecting a path.
    #[error("io error on {path}: {detail}")]
    Io {
        /// The affected path.
        path: PathBuf,
        /// The underlying IO error.
        #[source]
        detail: io::Error,
    },

    /// A `SQLite` error.
    #[error("sqlite error on {path}: {detail}")]
    Sqlite {
        /// The affected file.
        path: PathBuf,
        /// The underlying `SQLite` error.
        #[source]
        detail: rusqlite::Error,
    },
}

impl Error {
    /// Wrap a `rusqlite` error with path context; `NOTADB` becomes
    /// [`Error::NotASqliteDatabase`].
    pub(crate) fn from_sqlite(path: &Path, err: rusqlite::Error) -> Self {
        if let rusqlite::Error::SqliteFailure(failure, _) = &err {
            if failure.code == rusqlite::ErrorCode::NotADatabase {
                return Self::NotASqliteDatabase {
                    path: path.to_path_buf(),
                };
            }
        }
        Self::Sqlite {
            path: path.to_path_buf(),
            detail: err,
        }
    }
}
