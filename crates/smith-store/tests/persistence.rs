//! `smith-store` acceptance tests for issue #4 (`docs/architecture/03-persistence.md`).
//!
//! One test per acceptance criterion; names follow the issue's acceptance list:
//! REQ-PERS-004/005/006/009/010/011/017/018/019/020.
//!
//! Style: every test returns [`TestResult`] and uses the `ensure` macros —
//! repo lints deny `expect`/`unwrap`/`panic`/assertions-in-`Result`-fns.

use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;

use smith_store::{open, Error};

/// Test outcome: any helper or `ensure` failure fails the test via `?`.
type TestResult = Result<(), Box<dyn std::error::Error>>;

const SQLITE_MAGIC: &[u8; 16] = b"SQLite format 3\0";

/// Assertion that fails the test by returning `Err` (no panics).
macro_rules! ensure {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(format!($($arg)*).into());
        }
    };
}

/// Equality assertion that fails the test by returning `Err` (no panics).
macro_rules! ensure_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            return Err(format!(
                "assertion failed: `{}` == `{}`\n  left: {:?}\n right: {:?}",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            )
            .into());
        }
    };
    ($left:expr, $right:expr, $($arg:tt)*) => {
        if $left != $right {
            return Err(format!(
                "assertion failed: `{}` == `{}` ({})\n  left: {:?}\n right: {:?}",
                stringify!($left),
                stringify!($right),
                format_args!($($arg)*),
                $left,
                $right
            )
            .into());
        }
    };
}

/// A throwaway directory with its `.smith` file path; removed on drop.
struct Tmp {
    _dir: tempfile::TempDir,
    smith: PathBuf,
}

impl Tmp {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let smith = dir.path().join("model.smith");
        Ok(Self { _dir: dir, smith })
    }

    fn path(&self) -> &Path {
        &self.smith
    }
}

fn query_one<T: rusqlite::types::FromSql>(conn: &Connection, sql: &str) -> rusqlite::Result<T> {
    conn.query_row(sql, [], |row| row.get(0))
}

fn names(conn: &Connection, sql: &str) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare(sql)?;
    let mut rows = stmt.query([])?;
    let mut out = Vec::new();
    while let Some(row) = rows.next()? {
        out.push(row.get(0)?);
    }
    out.sort();
    Ok(out)
}

fn table_names(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    names(
        conn,
        "SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name",
    )
}

fn index_names(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    names(
        conn,
        "SELECT name FROM sqlite_master WHERE type = 'index' ORDER BY name",
    )
}

fn pragma(conn: &Connection, name: &str) -> rusqlite::Result<String> {
    query_one(conn, &format!("PRAGMA {name}"))
}

fn pragma_int(conn: &Connection, name: &str) -> rusqlite::Result<i64> {
    query_one(conn, &format!("PRAGMA {name}"))
}

/// Plain-SQL row writer: exercises REQ-PERS-018 inspectability (no Smith API).
fn insert_element(conn: &Connection, id: &str, kind: &str, name: &str) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO elements (id, kind, name, data, created_at, updated_at) \
         VALUES (?1, ?2, ?3, '{}', '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
        rusqlite::params![id, kind, name],
    )
}

/// A pre-V001 `.smith` file: Smith magic set, `smith_meta` at `version`, no schema.
fn make_pre_migration_file(path: &Path, version: &str) -> rusqlite::Result<()> {
    let conn = Connection::open(path)?;
    conn.execute_batch(&format!(
        "PRAGMA application_id = {}; \
         CREATE TABLE smith_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL); \
         INSERT INTO smith_meta (key, value) VALUES ('schema_version', '{version}');",
        smith_store::APPLICATION_ID
    ))
}

/// Whether a `.bak` file's `ui_state` contains a generation marker.
fn has_ui_marker(path: &Path, key: &str) -> rusqlite::Result<bool> {
    let conn = Connection::open(path)?;
    conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM ui_state WHERE key = ?1)",
        [key],
        |row| row.get::<_, bool>(0),
    )
}

/// Unwrap an expected `open` failure, failing the test with the store's debug
/// view if it succeeded.
fn open_err(path: &Path, context: &str) -> Result<Error, Box<dyn std::error::Error>> {
    match open(path) {
        Ok(store) => Err(format!("{context}: expected an error, got {store:?}").into()),
        Err(err) => Ok(err),
    }
}

// ---------------------------------------------------------------------------
// REQ-PERS-019: application_id magic written and verified
// ---------------------------------------------------------------------------

#[test]
fn application_id_magic_written_and_verified() -> TestResult {
    let tmp = Tmp::new()?;
    let store = open(tmp.path())?;
    let app_id: i64 = query_one(store.connection(), "PRAGMA application_id")?;
    ensure_eq!(app_id, smith_store::APPLICATION_ID);
    ensure_eq!(store.schema_version(), 1, "fresh file migrates to V001");
    drop(store);

    // Reopen succeeds: the magic is present and verified.
    let store = open(tmp.path())?;
    ensure_eq!(store.schema_version(), 1);
    Ok(())
}

#[test]
fn non_smith_sqlite_rejected_with_clear_error() -> TestResult {
    let tmp = Tmp::new()?;

    // A valid SQLite database that is NOT a Smith file (no application_id set).
    let plain = Connection::open(tmp.path())?;
    plain.execute_batch("CREATE TABLE t (x INTEGER); INSERT INTO t VALUES (1);")?;
    drop(plain);

    let err = open_err(tmp.path(), "plain SQLite file")?;
    let msg = err.to_string();
    ensure!(
        matches!(err, Error::NotASmithFile { .. }),
        "expected NotASmithFile, got: {err:?}"
    );
    ensure!(
        msg.contains("Smith") && msg.contains(&*tmp.path().to_string_lossy()),
        "clear error naming the file, got: {msg}"
    );

    // Non-SQLite garbage is rejected, too.
    fs::write(tmp.path(), b"this is not a database at all")?;
    let err = open_err(tmp.path(), "garbage bytes")?;
    ensure!(
        matches!(err, Error::NotASqliteDatabase { .. }),
        "expected NotASqliteDatabase, got: {err:?}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-011: WAL + synchronous=NORMAL + foreign_keys=ON
// ---------------------------------------------------------------------------

#[test]
fn open_configures_wal_synchronous_normal_and_foreign_keys() -> TestResult {
    let tmp = Tmp::new()?;
    let store = open(tmp.path())?;
    let conn = store.connection();

    ensure_eq!(pragma(conn, "journal_mode")?, "wal");
    ensure_eq!(pragma_int(conn, "synchronous")?, 1, "NORMAL = 1");
    ensure_eq!(query_one::<i64>(conn, "PRAGMA foreign_keys")?, 1);
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-004: V001 schema — all tables, indexes, and pragmas
// ---------------------------------------------------------------------------

#[test]
fn schema_v001_creates_all_tables_indexes_and_pragmas() -> TestResult {
    let tmp = Tmp::new()?;
    let store = open(tmp.path())?;
    let conn = store.connection();

    let tables = table_names(conn)?;
    for table in [
        "smith_meta",
        "elements",
        "ownership_closure",
        "relationships",
        "applied_stereotypes",
        "comments",
        "diagrams",
        "shape_views",
        "edge_views",
        "elements_fts",
        "ui_state",
    ] {
        ensure!(tables.contains(&table.to_string()), "missing table {table}");
    }

    let indexes = index_names(conn)?;
    for index in [
        "idx_elements_owner",
        "idx_elements_kind",
        "idx_closure_anc",
        "idx_closure_dec",
        "idx_rel_source",
        "idx_rel_target",
        "idx_rel_kind",
        "idx_stereo_element",
        "idx_comments_owner",
        "idx_shape_diagram",
        "idx_shape_element",
        "idx_edge_diagram",
    ] {
        ensure!(
            indexes.contains(&index.to_string()),
            "missing index {index}"
        );
    }

    // Pragmas that must hold on the opened connection.
    ensure_eq!(pragma(conn, "journal_mode")?, "wal");
    ensure_eq!(pragma_int(conn, "synchronous")?, 1, "NORMAL = 1");
    ensure_eq!(query_one::<i64>(conn, "PRAGMA foreign_keys")?, 1);

    // Schema sanity: the FTS5 virtual table and its shadow tables parse cleanly.
    ensure_eq!(query_one::<String>(conn, "PRAGMA integrity_check")?, "ok");
    let mut stmt = conn.prepare("PRAGMA foreign_key_check")?;
    let mut rows = stmt.query([])?;
    ensure!(rows.next()?.is_none(), "unexpected FK violation");
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-006: shape_views element index doubles as the coverage index
// ---------------------------------------------------------------------------

#[test]
fn shape_views_element_index_serves_as_coverage_index() -> TestResult {
    let tmp = Tmp::new()?;
    let store = open(tmp.path())?;
    let conn = store.connection();

    insert_element(conn, "pkg-1", "Package", "root")?;
    insert_element(conn, "cls-1", "Class", "covered")?;
    insert_element(conn, "cls-2", "Class", "uncovered")?;
    conn.execute(
        "INSERT INTO diagrams (id, name, kind, owner_id) \
         VALUES ('d1', 'Main', 'Class', 'pkg-1')",
        [],
    )?;
    conn.execute(
        "INSERT INTO shape_views (id, diagram_id, model_element_id, bounds) \
         VALUES ('s1', 'd1', 'cls-1', '{\"x\":0,\"y\":0,\"w\":100,\"h\":50}')",
        [],
    )?;

    // Coverage analysis (REQ-DI-015): covered set via the element index,
    // uncovered via set difference against elements.
    let mut plan_stmt =
        conn.prepare("EXPLAIN QUERY PLAN SELECT model_element_id FROM shape_views")?;
    let mut plan_rows = plan_stmt.query([])?;
    let mut plan = String::new();
    while let Some(row) = plan_rows.next()? {
        plan.push_str(&row.get::<_, String>(3)?);
        plan.push('\n');
    }
    ensure!(
        plan.contains("idx_shape_element"),
        "coverage scan must use the element index, got: {plan}"
    );
    let covered = names(conn, "SELECT DISTINCT model_element_id FROM shape_views")?;
    let uncovered = names(
        conn,
        "SELECT id FROM elements \
         WHERE id NOT IN (SELECT model_element_id FROM shape_views) ORDER BY id",
    )?;
    ensure_eq!(covered, vec!["cls-1".to_string()]);
    ensure_eq!(uncovered, vec!["cls-2".to_string(), "pkg-1".to_string()]);
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-005: data blobs are validated JSON, serde round-trips
// ---------------------------------------------------------------------------

mod blob_round_trip {
    use serde::{Deserialize, Serialize};

    use smith_store::data::{deserialize_data, serialize_data};

    use super::TestResult;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct ElementData {
        #[serde(default)]
        is_abstract: bool,
        attributes: Vec<String>,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct RelationshipData {
        ends: Vec<String>,
    }

    #[test]
    fn element_and_relationship_data_blobs_round_trip_validated_json() -> TestResult {
        let element = ElementData {
            is_abstract: true,
            attributes: vec!["id: Integer".into(), "name: String".into()],
        };
        let blob = serialize_data(&element)?;
        // The blob is real JSON, parseable independently of Smith.
        let parsed: serde_json::Value = serde_json::from_str(&blob)?;
        ensure_eq!(parsed["attributes"][1], "name: String");
        let round: ElementData = deserialize_data(&blob)?;
        ensure_eq!(round, element);

        let relationship = RelationshipData {
            ends: vec!["1".into(), "0..*".into()],
        };
        let blob = serialize_data(&relationship)?;
        let round: RelationshipData = deserialize_data(&blob)?;
        ensure_eq!(round, relationship);
        Ok(())
    }

    #[test]
    fn raw_unvalidated_json_is_never_trusted() -> TestResult {
        // Truncated JSON must not deserialize.
        ensure!(
            deserialize_data::<ElementData>(r#"{"attributes": ["x""#).is_err(),
            "truncated JSON must be rejected"
        );
        // Valid JSON of the wrong shape must not deserialize either.
        ensure!(
            deserialize_data::<ElementData>(r#"{"unrelated": 1}"#).is_err(),
            "wrong-shape JSON must be rejected"
        );
        Ok(())
    }

    #[test]
    fn store_insert_rejects_invalid_json_blobs() -> TestResult {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("model.smith");
        let store = smith_store::open(&path)?;
        let conn = store.connection();

        let result = conn.execute(
            "INSERT INTO elements (id, kind, name, data, created_at, updated_at) \
             VALUES ('e1', 'Class', 'C', 'not json', '2026-01-01T00:00:00Z', \
                     '2026-01-01T00:00:00Z')",
            [],
        );
        let Err(err) = result else {
            return Err("invalid JSON blob must be rejected".into());
        };
        ensure!(
            err.to_string().contains("CHECK constraint"),
            "clear error, got: {err}"
        );
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// REQ-PERS-009: migration runner — pending migrations in a transaction
// ---------------------------------------------------------------------------

#[test]
fn migration_runner_applies_pending_in_transaction_and_bumps_version() -> TestResult {
    // A file created before V001 existed: smith_meta holds schema_version 0 and
    // the schema tables are absent.
    let tmp = Tmp::new()?;
    make_pre_migration_file(tmp.path(), "0")?;

    let store = open(tmp.path())?;
    ensure_eq!(store.schema_version(), 1);
    let conn = store.connection();
    let tables = table_names(conn)?;
    ensure!(
        tables.contains(&"elements".to_string()) && tables.contains(&"relationships".to_string()),
        "pending V001 applied: {tables:?}"
    );
    let version: String = query_one(
        conn,
        "SELECT value FROM smith_meta WHERE key = 'schema_version'",
    )?;
    ensure_eq!(version, "1".to_string(), "version bumped in smith_meta");

    // Reopen: nothing pending, version unchanged.
    drop(store);
    let store = open(tmp.path())?;
    ensure_eq!(store.schema_version(), 1);
    Ok(())
}

#[test]
fn failed_migration_aborts_open_with_clear_error() -> TestResult {
    // A file at schema_version 0 whose contents collide with V001's DDL: the
    // migration fails, open aborts with a clear error, and the file is left
    // untouched (transaction rolled back).
    let tmp = Tmp::new()?;
    let conn = Connection::open(tmp.path())?;
    conn.execute_batch(&format!(
        "PRAGMA application_id = {}; \
         CREATE TABLE elements (bogus TEXT); \
         CREATE TABLE smith_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL); \
         INSERT INTO smith_meta (key, value) VALUES ('schema_version', '0');",
        smith_store::APPLICATION_ID
    ))?;
    drop(conn);

    let err = open_err(tmp.path(), "colliding schema")?;
    let msg = err.to_string();
    ensure!(
        matches!(err, Error::MigrationFailed { version: 1, .. }),
        "expected MigrationFailed for V001, got: {err:?}"
    );
    ensure!(
        msg.to_lowercase().contains("migrat"),
        "clear error, got: {msg}"
    );

    // The failed migration left the file alone: reopens fail the same way until
    // the collision is fixed forward (REQ-PERS-010).
    ensure!(open(tmp.path()).is_err(), "reopen must keep failing");
    Ok(())
}

#[test]
fn corrupt_schema_version_aborts_open_with_clear_error() -> TestResult {
    let tmp = Tmp::new()?;
    make_pre_migration_file(tmp.path(), "not-a-number")?;

    let err = open_err(tmp.path(), "corrupt version")?;
    ensure!(
        matches!(err, Error::CorruptSchemaVersion { .. }),
        "expected CorruptSchemaVersion, got: {err:?}"
    );
    ensure!(
        err.to_string().contains("schema_version"),
        "names the corrupt key, got: {err}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-010: migration registry is append-only
// ---------------------------------------------------------------------------

#[test]
fn migration_registry_is_append_only() -> TestResult {
    use smith_store::migrations;

    let registry = migrations::REGISTRY;
    ensure!(!registry.is_empty(), "V001 must ship");

    for (i, migration) in registry.iter().enumerate() {
        let expected = u32::try_from(i)? + 1;
        ensure_eq!(
            migration.version,
            expected,
            "versions are dense, 1-based, in order"
        );
    }
    // No down-migrations exist: no migration drops anything.
    let has_down = registry
        .iter()
        .any(|m| m.sql.to_lowercase().contains("drop "));
    ensure!(!has_down, "append-only: no migration drops schema objects");
    // The registry exposes no down/revert API: `Migration` is version + forward
    // SQL only, and the current version is the last registered migration.
    ensure_eq!(
        smith_store::current_schema_version(),
        u32::try_from(registry.len())?
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-017: single SQLite DB, checkpointed on close
// ---------------------------------------------------------------------------

#[test]
fn smith_file_is_single_sqlite_db_checkpointed_on_close() -> TestResult {
    let tmp = Tmp::new()?;
    let store = open(tmp.path())?;
    insert_element(store.connection(), "e1", "Class", "C")?;
    // WAL sidecar exists while the WAL-mode database is open and dirty.
    ensure!(
        tmp.path().with_extension("smith-wal").exists(),
        "-wal sidecar while open"
    );
    store.close()?; // clean close: checkpoint folds the WAL into the main file

    ensure!(
        !tmp.path().with_extension("smith-wal").exists(),
        "-wal folded into the main file on close"
    );
    let header = fs::read(tmp.path())?;
    ensure_eq!(&header[0..16], SQLITE_MAGIC, "single SQLite DB");
    ensure!(fs::metadata(tmp.path())?.len() > 0, "main file non-empty");

    // The row is durable in the main file alone (no sidecar needed).
    let conn = Connection::open(tmp.path())?;
    let name: String = query_one(&conn, "SELECT name FROM elements WHERE id = 'e1'")?;
    ensure_eq!(name, "C".to_string());
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-018: inspectable by plain sqlite3
// ---------------------------------------------------------------------------

#[test]
fn smith_file_is_inspectable_by_plain_sqlite() -> TestResult {
    let tmp = Tmp::new()?;
    let store = open(tmp.path())?;
    insert_element(store.connection(), "e1", "Package", "root")?;
    drop(store);

    // Standard SQLite header.
    let header = fs::read(tmp.path())?;
    ensure_eq!(&header[0..16], SQLITE_MAGIC);

    // `.tables`-equivalent listing via plain SQL, no Smith code involved.
    let conn = Connection::open(tmp.path())?;
    let tables = table_names(&conn)?;
    ensure!(tables.contains(&"elements".to_string()), "elements listed");
    let kind: String = query_one(&conn, "SELECT kind FROM elements WHERE id = 'e1'")?;
    ensure_eq!(kind, "Package".to_string());
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-020: .bak rotation, keeping the last 3
// ---------------------------------------------------------------------------

#[test]
fn open_rotates_smith_bak_keeping_last_three() -> TestResult {
    let tmp = Tmp::new()?;
    let smith = tmp.path().to_path_buf();
    let backup = |n: u32| smith.with_extension(format!("smith.bak.{n}"));

    // Generation 1: create the file and stamp a marker. No prior version
    // exists → no backup yet.
    let store = open(&smith)?;
    store.connection().execute(
        "INSERT INTO ui_state (key, value) VALUES ('gen-1', '1')",
        [],
    )?;
    drop(store);
    ensure!(!backup(1).exists(), "no backup of a brand-new file");

    // Generations 2..=5: each open backs up the previous state, then stamps a marker.
    for generation in 2..=5u32 {
        let store = open(&smith)?;
        store.connection().execute(
            &format!("INSERT INTO ui_state (key, value) VALUES ('gen-{generation}', '1')"),
            [],
        )?;
        drop(store);
    }

    // Only the last 3 backups kept: bak.4 (oldest) dropped.
    ensure!(!backup(4).exists(), "only the last 3 kept");
    // bak.1 is the newest backup = state before the 5th open (has gen-4).
    ensure!(has_ui_marker(&backup(1), "gen-4")?, "bak.1 has gen-4");
    ensure!(
        !has_ui_marker(&backup(1), "gen-5")?,
        "backup precedes the open"
    );
    // bak.2 = state before the 4th open (has gen-3).
    ensure!(has_ui_marker(&backup(2), "gen-3")?, "bak.2 has gen-3");
    ensure!(!has_ui_marker(&backup(2), "gen-4")?, "bak.2 lacks gen-4");
    // bak.3 = state before the 3rd open (has gen-2).
    ensure!(has_ui_marker(&backup(3), "gen-2")?, "bak.3 has gen-2");
    ensure!(!has_ui_marker(&backup(3), "gen-3")?, "bak.3 lacks gen-3");

    // Backups are valid Smith files themselves (openable).
    drop(open(&backup(1))?);
    Ok(())
}

// ---------------------------------------------------------------------------
// Store handle: connection access + schema version
// ---------------------------------------------------------------------------

#[test]
fn store_exposes_connection_and_schema_version() -> TestResult {
    let tmp = Tmp::new()?;
    let store = open(tmp.path())?;
    ensure_eq!(store.schema_version(), 1);
    ensure_eq!(
        query_one::<i64>(store.connection(), "PRAGMA application_id")?,
        smith_store::APPLICATION_ID
    );
    ensure_eq!(store.path(), tmp.path());
    Ok(())
}
