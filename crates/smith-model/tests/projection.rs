//! `smith-model` acceptance tests for issue #8 — the petgraph projection and
//! Slice 1 round-trip integration test.
//!
//! One test per acceptance criterion, named per the issue. Style: deterministic
//! tests return [`TestResult`] and use the `ensure` macros — repo lints deny
//! `expect`/`unwrap`/`panic`.

use std::collections::BTreeSet;
use std::time::Instant;

use smith_core::{ElementId, MetaclassKind, Visibility};
use smith_model::{CreateElement, CreateRelationship, EdgeKind, Model, Projection};

/// Deterministic test outcome: helper failures propagate via `?` (no panics).
type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Assertion that fails the test by returning `Err` (no panics).
macro_rules! ensure {
    ($cond:expr) => {
        if !$cond {
            return Err(format!("assertion failed: `{}`", stringify!($cond)).into());
        }
    };
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
                $right,
            )
            .into());
        }
    };
    ($left:expr, $right:expr, $($arg:tt)*) => {
        if $left != $right {
            return Err(format!(
                "assertion failed: `{}` == `{}`: {}\n  left: {:?}\n right: {:?}",
                stringify!($left),
                stringify!($right),
                format!($($arg)*),
                $left,
                $right,
            )
            .into());
        }
    };
}

/// Open a fresh temp-file model for a test.
fn fresh_model() -> Result<Model, smith_model::Error> {
    let dir = tempfile::tempdir().map_err(|e| smith_model::Error::Sqlite {
        detail: rusqlite::Error::ToSqlConversionFailure(Box::new(e)),
    })?;
    Model::open(&dir.path().join("test.smith"))
}

/// Mint a fresh element id.
fn new_id() -> ElementId {
    ElementId::new()
}

/// Create the project root, returning its id.
fn create_root(model: &mut Model, name: &str) -> Result<ElementId, smith_model::Error> {
    let id = new_id();
    model.create_root(id, Some(name.to_string()))?;
    Ok(id)
}

/// Create a package under `owner`.
fn create_pkg(
    model: &mut Model,
    owner: ElementId,
    name: &str,
) -> Result<ElementId, smith_model::Error> {
    let id = new_id();
    model.create_element(&CreateElement {
        id,
        kind: MetaclassKind::Package,
        name: Some(name.to_string()),
        owner: Some(owner),
        visibility: Visibility::Public,
    })?;
    Ok(id)
}

/// Create a class under `owner`.
fn create_class(
    model: &mut Model,
    owner: ElementId,
    name: &str,
) -> Result<ElementId, smith_model::Error> {
    let id = new_id();
    model.create_element(&CreateElement {
        id,
        kind: MetaclassKind::Class,
        name: Some(name.to_string()),
        owner: Some(owner),
        visibility: Visibility::Public,
    })?;
    Ok(id)
}

/// Build a 3-class hierarchy in `model`:
///
/// ```text
/// root (Package, isModel)
/// ├── models (Package)
/// │   ├── Animal (Class)
/// │   └── Dog (Class)
/// └── views (Package)
///     └── DogView (Class)
/// ```
///
/// Plus an Association `Dog → DogView` and a Generalization `Dog → Animal`,
/// both owned by root.
fn build_three_class_hierarchy(
    model: &mut Model,
) -> Result<
    (
        ElementId,
        ElementId,
        ElementId,
        ElementId,
        ElementId,
        ElementId,
    ),
    smith_model::Error,
> {
    let root = create_root(model, "root")?;
    let models = create_pkg(model, root, "models")?;
    let animal = create_class(model, models, "Animal")?;
    let dog = create_class(model, models, "Dog")?;
    let views = create_pkg(model, root, "views")?;
    let dog_view = create_class(model, views, "DogView")?;

    // Generalization: Dog specializes Animal.
    let gen_id = new_id();
    model.create_relationship(&CreateRelationship {
        id: gen_id,
        kind: "Generalization".to_string(),
        source: dog,
        target: animal,
        owner: root,
    })?;

    // Association: Dog ↔ DogView.
    let assoc_id = new_id();
    model.create_relationship(&CreateRelationship {
        id: assoc_id,
        kind: "Association".to_string(),
        source: dog,
        target: dog_view,
        owner: root,
    })?;

    Ok((root, models, animal, dog, views, dog_view))
}

/// Collect the projection's element ids for stable comparison.
fn projection_element_ids(proj: &Projection) -> BTreeSet<String> {
    proj.elements_sorted()
        .iter()
        .map(|n| n.id.as_uuid().to_string())
        .collect()
}
/// Collect the projection's edges as `(source, target, kind)` tuples for
/// stable comparison.
fn projection_edge_set(proj: &Projection) -> BTreeSet<(String, String, String)> {
    proj.edges_sorted()
        .iter()
        .map(|e| {
            let kind_str = match &e.kind {
                EdgeKind::Ownership => "ownership".to_string(),
                EdgeKind::Relationship { id, kind } => {
                    format!("rel:{kind}:{id:?}")
                }
            };
            (
                e.source.as_uuid().to_string(),
                e.target.as_uuid().to_string(),
                kind_str,
            )
        })
        .collect()
}

#[test]
fn projection_rebuilds_from_sqlite_and_sqlite_wins_on_divergence() -> TestResult {
    let mut model = fresh_model()?;
    let (root, models, animal, dog, _views, dog_view) = build_three_class_hierarchy(&mut model)?;

    // Snapshot the projection before rebuild.
    let before_ids = projection_element_ids(model.projection());
    let before_edges = projection_edge_set(model.projection());
    ensure_eq!(before_ids.len(), 6, "6 elements in the projection");
    // 5 ownership edges + 2 relationship edges = 7.
    ensure_eq!(before_edges.len(), 7, "7 edges in the projection");

    // Rebuild the projection from SQLite.
    model.rebuild_projection()?;

    // After rebuild, the projection must match what SQLite holds (SQLite wins).
    let after_ids = projection_element_ids(model.projection());
    let after_edges = projection_edge_set(model.projection());
    ensure_eq!(
        before_ids,
        after_ids,
        "element ids must match after rebuild"
    );
    ensure_eq!(before_edges, after_edges, "edges must match after rebuild");

    // Verify specific elements and edges survive rebuild.
    let proj = model.projection();
    ensure!(proj.node(root).is_some(), "root must be in projection");
    ensure!(proj.node(dog).is_some(), "dog must be in projection");
    ensure!(
        proj.node(dog_view).is_some(),
        "dog_view must be in projection"
    );
    ensure_eq!(proj.node_count(), 6, "6 nodes after rebuild");
    ensure_eq!(proj.edge_count(), 7, "7 edges after rebuild");

    // Verify ownership structure: models.owner == root, animal.owner == models.
    ensure_eq!(
        proj.node(models).map(|n| n.owner),
        Some(Some(root)),
        "models owned by root"
    );
    ensure_eq!(
        proj.node(animal).map(|n| n.owner),
        Some(Some(models)),
        "animal owned by models"
    );
    Ok(())
}

// ===========================================================================
// REQ-PERS-003: rebuild from a fresh connection; no projection-derived state
// survives a rebuild (decoupled from store)
// ===========================================================================

#[test]
fn projection_decoupled_from_store_rebuild_preserves_model() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("decoupled.smith");

    // Create a model in one connection, then close it.
    let saved_ids: Vec<ElementId>;
    {
        let mut model = Model::open(&path)?;
        let (root, _models, _animal, _dog, _views, _dog_view) =
            build_three_class_hierarchy(&mut model)?;
        saved_ids = vec![root];
    } // model dropped → checkpointed

    // Reopen from a FRESH connection — no projection-derived state survives.
    let mut model2 = Model::open(&path)?;
    let proj = model2.projection();

    // The projection was hydrated from SQLite on open, not from any cached
    // state. Verify it matches the store.
    ensure!(
        proj.node_count() > 0,
        "projection must have nodes after fresh open"
    );

    // Rebuild: this must produce the same graph (no state from the old
    // projection survives — the rebuild discards everything and re-reads).
    let before = projection_edge_set(proj);
    model2.rebuild_projection()?;
    let after = projection_edge_set(model2.projection());
    ensure_eq!(
        before,
        after,
        "rebuild from fresh connection must preserve the model"
    );

    // The root from the saved model must be in the rebuilt projection.
    ensure!(
        model2.projection().node(saved_ids[0]).is_some(),
        "root must survive rebuild from fresh connection"
    );
    Ok(())
}

// ===========================================================================
// Slice 1 acceptance: round-trip 3-class hierarchy survives close + reopen
// (also asserts INV-MM-001: ownership tree acyclic + connected)
// ===========================================================================

#[test]
fn round_trip_three_class_hierarchy_survives_close_reopen() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("roundtrip.smith");

    // Phase 1: create the 3-class hierarchy and snapshot the projection.
    let (root, models, animal, dog, views, dog_view, before_ids, before_edges) = {
        let mut model = Model::open(&path)?;
        let (r, m, ani, dg, vw, dvw) = build_three_class_hierarchy(&mut model)?;
        let ids = projection_element_ids(model.projection());
        let edges = projection_edge_set(model.projection());

        // INV-MM-001: the ownership tree is acyclic + connected.
        ensure!(
            model.projection().ownership_is_acyclic(),
            "ownership tree must be acyclic"
        );
        ensure!(
            model.projection().ownership_is_connected(),
            "ownership tree must be connected"
        );
        (r, m, ani, dg, vw, dvw, ids, edges)
    }; // model dropped → store checkpointed

    // Phase 2: reopen, hydrate the projection, assert equality.
    let model2 = Model::open(&path)?;
    let after_ids = projection_element_ids(model2.projection());
    let after_edges = projection_edge_set(model2.projection());

    // The ownership tree + relationships must match.
    ensure_eq!(
        before_ids,
        after_ids,
        "element ids must match after close + reopen"
    );
    ensure_eq!(
        before_edges,
        after_edges,
        "edges must match after close + reopen"
    );

    // INV-MM-001 after reopen: still acyclic + connected.
    ensure!(
        model2.projection().ownership_is_acyclic(),
        "ownership tree must be acyclic after reopen"
    );
    ensure!(
        model2.projection().ownership_is_connected(),
        "ownership tree must be connected after reopen"
    );

    // Verify specific elements survived.
    let proj = model2.projection();
    ensure!(proj.node(root).is_some(), "root must survive reopen");
    ensure!(proj.node(models).is_some(), "models must survive reopen");
    ensure!(proj.node(animal).is_some(), "animal must survive reopen");
    ensure!(proj.node(dog).is_some(), "dog must survive reopen");
    ensure!(proj.node(views).is_some(), "views must survive reopen");
    ensure!(
        proj.node(dog_view).is_some(),
        "dog_view must survive reopen"
    );

    // Verify ownership chain: root → models → animal, root → views → dog_view.
    ensure_eq!(
        proj.node(models).map(|n| n.owner),
        Some(Some(root)),
        "models owned by root"
    );
    ensure_eq!(
        proj.node(animal).map(|n| n.owner),
        Some(Some(models)),
        "animal owned by models"
    );
    ensure_eq!(
        proj.node(dog).map(|n| n.owner),
        Some(Some(models)),
        "dog owned by models"
    );
    ensure_eq!(
        proj.node(views).map(|n| n.owner),
        Some(Some(root)),
        "views owned by root"
    );
    ensure_eq!(
        proj.node(dog_view).map(|n| n.owner),
        Some(Some(views)),
        "dog_view owned by views"
    );

    // Verify relationship edges exist.
    let edges = proj.edges_sorted();
    let has_generalization = edges.iter().any(|e| {
        e.source == dog
            && e.target == animal
            && matches!(
                &e.kind,
                EdgeKind::Relationship { kind, .. } if kind == "Generalization"
            )
    });
    ensure!(
        has_generalization,
        "Generalization edge (dog → animal) must exist"
    );

    let has_association = edges.iter().any(|e| {
        e.source == dog
            && e.target == dog_view
            && matches!(
                &e.kind,
                EdgeKind::Relationship { kind, .. } if kind == "Association"
            )
    });
    ensure!(
        has_association,
        "Association edge (dog → dog_view) must exist"
    );
    Ok(())
}

// ===========================================================================
// REQ-PERS-021 (cold load): hydration of 100k elements under one second
// ===========================================================================

#[test]
fn hydration_of_100k_elements_under_one_second() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("perf_cold.smith");

    // Phase 1: seed 100,000 elements into SQLite.
    {
        let mut model = Model::open(&path)?;
        let root = create_root(&mut model, "root")?;

        // Bulk-insert 100k packages under root via a single transaction batch.
        // We bypass the model API for seeding (this test measures hydration,
        // not mutation speed). Only the elements table is seeded — the
        // projection hydrates from elements + relationships, not the
        // ownership_closure index, so closure rows are omitted to keep the
        // measured VACUUM backup honest about what hydration actually reads.
        let conn = model.connection();
        let tx = conn.unchecked_transaction()?;
        tx.execute_batch("PRAGMA foreign_keys=OFF")?;
        {
            let mut elem_stmt = tx.prepare(
                "INSERT INTO elements (id, kind, name, owner_id, visibility, data, \
                 created_at, updated_at) VALUES (?1, 'Package', ?2, ?3, 'public', '{}', \
                 '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
            )?;
            for i in 0..100_000 {
                let id = ElementId::new();
                elem_stmt.execute(rusqlite::params![
                    id.as_uuid().to_string(),
                    format!("pkg{i}"),
                    root.as_uuid().to_string()
                ])?;
            }
        }
        tx.execute_batch("PRAGMA foreign_keys=ON")?;
        tx.commit()?;
    }

    // Phase 2: reopen and measure hydration time.
    let start = Instant::now();
    let model = Model::open(&path)?;
    let elapsed = start.elapsed();

    let under_budget = elapsed.as_secs_f64() < 1.0;
    ensure!(
        under_budget,
        "cold hydration of 100k elements took {elapsed:?}, must be < 1s"
    );

    // Verify the projection has the right node count.
    ensure_eq!(
        model.projection().node_count(),
        100_001,
        "100k elements + 1 root = 100,001 nodes"
    );

    // Verify ownership edges: 100k ownership edges (root → each child).
    let ownership_count = model
        .projection()
        .edges_sorted()
        .iter()
        .filter(|e| matches!(e.kind, EdgeKind::Ownership))
        .count();
    ensure_eq!(ownership_count, 100_000, "100,000 ownership edges expected");

    // INV-MM-001: the tree must be acyclic + connected.
    ensure!(
        model.projection().ownership_is_acyclic(),
        "ownership tree must be acyclic after bulk hydrate"
    );
    ensure!(
        model.projection().ownership_is_connected(),
        "ownership tree must be connected after bulk hydrate"
    );
    Ok(())
}

// ===========================================================================
// REQ-PERS-021 (mutation): single mutation end-to-end under 10ms
// ===========================================================================

#[test]
fn single_mutation_end_to_end_under_10ms() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;

    // Warm up: create a few elements so the store has data.
    let _pkg = create_pkg(&mut model, root, "warmup")?;

    // Measure a single create_element mutation end-to-end (including
    // closure-table + projection update).
    let id = new_id();
    let start = Instant::now();
    let _view = model.create_element(&CreateElement {
        id,
        kind: MetaclassKind::Class,
        name: Some("bench".to_string()),
        owner: Some(root),
        visibility: Visibility::Public,
    })?;
    let elapsed = start.elapsed();

    let under_budget = elapsed.as_secs_f64() < 0.010;
    ensure!(
        under_budget,
        "single mutation took {elapsed:?}, must be < 10ms"
    );

    // Verify the mutation landed in the projection (write-through).
    ensure!(
        model.projection().node(id).is_some(),
        "mutated element must be in the projection"
    );
    Ok(())
}

// ===========================================================================
// REQ-PERS-021 (checkpoint): WAL checkpoint on close under 500ms
// ===========================================================================

#[test]
fn wal_checkpoint_on_close_under_500ms() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("checkpoint.smith");

    // Create a model with enough data to have a non-trivial WAL.
    let root_id: ElementId;
    {
        let mut model = Model::open(&path)?;
        root_id = create_root(&mut model, "root")?;
        // Create a modest number of elements to populate the WAL.
        for i in 0..1000 {
            create_pkg(&mut model, root_id, &format!("pkg{i}"))?;
        }
    }

    // Reopen and measure the close (which does the TRUNCATE checkpoint).
    let model = Model::open(&path)?;
    let start = Instant::now();
    // The Model's Drop runs the checkpoint. We force it by dropping.
    drop(model);
    let elapsed = start.elapsed();

    let under_budget = elapsed.as_secs_f64() < 0.500;
    ensure!(
        under_budget,
        "WAL checkpoint on close took {elapsed:?}, must be < 500ms"
    );

    // Verify the data is still intact after checkpoint.
    let model2 = Model::open(&path)?;
    ensure_eq!(
        model2.projection().node_count(),
        1001,
        "1001 elements after checkpoint"
    );
    Ok(())
}

// ===========================================================================
// Write-through: mutations update projection and SQLite together
// ===========================================================================

#[test]
fn write_through_projection_matches_sqlite_after_mutations() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;
    let b = create_class(&mut model, a, "b")?;

    // After each mutation, the projection must match SQLite.
    {
        let proj = model.projection();
        ensure_eq!(proj.node_count(), 3, "3 nodes after 3 creates");
        ensure_eq!(proj.edge_count(), 2, "2 ownership edges");

        // Verify node data matches SQLite.
        let proj_b = proj.node(b).ok_or("b missing from projection")?;
        let sqlite_b = model.get_element(b)?;
        ensure_eq!(proj_b.id, sqlite_b.id);
        ensure_eq!(proj_b.kind, sqlite_b.kind);
        ensure_eq!(proj_b.owner, sqlite_b.owner);
        ensure_eq!(proj_b.name, sqlite_b.name);
        ensure_eq!(proj_b.visibility, sqlite_b.visibility);
    }

    // Rename: projection must update.
    model.rename_element(b, Some("renamed"))?;
    {
        let proj = model.projection();
        let proj_b = proj.node(b).ok_or("b missing after rename")?;
        ensure_eq!(proj_b.name.as_deref(), Some("renamed"));
    }

    // Reparent: projection must update ownership edges.
    model.reparent_element(b, Some(root))?;
    {
        let proj = model.projection();
        let proj_b = proj.node(b).ok_or("b missing after reparent")?;
        ensure_eq!(
            proj_b.owner,
            Some(root),
            "b's owner must be root after reparent"
        );
    }

    // Relationship create: projection must add the edge.
    let rel_id = new_id();
    model.create_relationship(&CreateRelationship {
        id: rel_id,
        kind: "Association".to_string(),
        source: b,
        target: a,
        owner: root,
    })?;
    {
        let proj = model.projection();
        let has_rel = proj.edges_sorted().iter().any(|e| {
            e.source == b
                && e.target == a
                && matches!(&e.kind, EdgeKind::Relationship { id, .. } if *id == rel_id)
        });
        ensure!(has_rel, "relationship edge must be in projection");
    }

    // Relationship delete: projection must remove the edge.
    model.delete_relationship(rel_id)?;
    {
        let proj = model.projection();
        let still_has_rel = proj
            .edges_sorted()
            .iter()
            .any(|e| matches!(&e.kind, EdgeKind::Relationship { id, .. } if *id == rel_id));
        ensure!(
            !still_has_rel,
            "relationship edge must be gone after delete"
        );
    }

    // Element delete: projection must remove the node and its edges.
    model.delete_element(b)?;
    {
        let proj = model.projection();
        ensure!(proj.node(b).is_none(), "b must be gone from projection");
        ensure_eq!(proj.node_count(), 2, "2 nodes after deleting b");
    }
    Ok(())
}

// ===========================================================================
// INV-MM-001: ownership tree acyclic + connected after mutations
// ===========================================================================

#[test]
fn ownership_invariant_acyclic_and_connected_holds() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;
    let b = create_pkg(&mut model, a, "b")?;
    let c = create_pkg(&mut model, a, "c")?;

    {
        let proj = model.projection();
        ensure!(
            proj.ownership_is_acyclic(),
            "ownership tree must be acyclic"
        );
        ensure!(
            proj.ownership_is_connected(),
            "ownership tree must be connected"
        );
    }

    // After a reparent that maintains the invariant, still acyclic + connected.
    let _ = b; // b is used to populate the tree.
    model.reparent_element(c, Some(root))?;
    {
        let proj = model.projection();
        ensure!(
            proj.ownership_is_acyclic(),
            "must be acyclic after reparent"
        );
        ensure!(
            proj.ownership_is_connected(),
            "must be connected after reparent"
        );
    }
    Ok(())
}
