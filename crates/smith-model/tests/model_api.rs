//! `smith-model` acceptance tests for issue #6 — the Model API CRUD surface.
//!
//! One test per acceptance criterion, named per the issue. Style: deterministic
//! tests return [`TestResult`] and use the `ensure` macros — repo lints deny
//! `expect`/`unwrap`/`panic`.

use std::collections::BTreeSet;

use smith_core::{ElementId, MetaclassKind, Visibility};
use smith_model::{CreateElement, CreateRelationship, EdgeKind, Error, Model, Projection};

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
fn fresh_model() -> Result<Model, Error> {
    let dir = tempfile::tempdir().map_err(|e| Error::Sqlite {
        detail: rusqlite::Error::ToSqlConversionFailure(Box::new(e)),
    })?;
    Model::open(&dir.path().join("test.smith"))
}

/// Mint a fresh element id.
fn new_id() -> ElementId {
    ElementId::new()
}

/// Create the project root, returning its id.
fn create_root(model: &mut Model, name: &str) -> Result<ElementId, Error> {
    let id = new_id();
    model.create_root(id, Some(name.to_string()))?;
    Ok(id)
}

/// Create a package under `owner`.
fn create_pkg(model: &mut Model, owner: ElementId, name: &str) -> Result<ElementId, Error> {
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
fn create_class(model: &mut Model, owner: ElementId, name: &str) -> Result<ElementId, Error> {
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

// ===========================================================================
// REQ-MM-002: owner tree is acyclic and connected after mutations
// (incl. reparent_into_own_subtree_is_rejected)
// ===========================================================================

#[test]
fn owner_tree_is_acyclic_and_connected_after_mutations() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;
    let b = create_pkg(&mut model, a, "b")?;
    let c = create_pkg(&mut model, b, "c")?;

    // After creating root -> a -> b -> c, every element is reachable from root.
    {
        let conn = model.connection();
        let all_ids: BTreeSet<String> = conn
            .prepare("SELECT id FROM elements")?
            .query_map([], |r| r.get::<_, String>(0))?
            .collect::<rusqlite::Result<BTreeSet<String>>>()?;
        ensure_eq!(all_ids.len(), 4);
    }

    // Every element (except root) has exactly one owner; root has none.
    let root_view = model.get_element(root)?;
    ensure!(root_view.owner.is_none(), "root must have no owner");
    for id in [a, b, c] {
        let view = model.get_element(id)?;
        ensure!(view.owner.is_some(), "element {id:?} must have an owner");
    }

    // Reparent b from a to root: now root -> a, root -> b -> c.
    model.reparent_element(b, Some(root))?;
    let b_view = model.get_element(b)?;
    ensure_eq!(b_view.owner, Some(root));

    // c is still under b.
    let c_view = model.get_element(c)?;
    ensure_eq!(c_view.owner, Some(b));

    // The tree is still connected: c's ancestors include root.
    {
        let conn = model.connection();
        let c_ancestors = smith_store::closure::ancestors(conn, &c.as_uuid().to_string())?;
        let ancestor_ids: BTreeSet<String> = c_ancestors.into_iter().map(|r| r.id).collect();
        ensure!(
            ancestor_ids.contains(&root.as_uuid().to_string()),
            "root must be an ancestor of c after reparent"
        );
    }
    Ok(())
}

#[test]
fn reparent_into_own_subtree_is_rejected() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;
    let b = create_pkg(&mut model, a, "b")?;
    let c = create_pkg(&mut model, b, "c")?;

    // Reparenting a into c (its own descendant) must be rejected.
    let err = model.reparent_element(a, Some(c));
    let err = err
        .err()
        .ok_or("reparent into own subtree should have returned an error")?;
    ensure!(
        matches!(err, Error::ReparentIntoOwnSubtree { element, new_owner }
            if element == a && new_owner == c),
        "expected ReparentIntoOwnSubtree, got {err:?}"
    );

    // Reparenting a into b (its own child) must be rejected.
    let err = model.reparent_element(a, Some(b)).err();
    ensure!(
        matches!(err, Some(Error::ReparentIntoOwnSubtree { .. })),
        "reparent into own child should fail"
    );

    // Reparenting a into a (itself) must be rejected.
    let err = model.reparent_element(a, Some(a)).err();
    ensure!(
        matches!(err, Some(Error::ReparentIntoOwnSubtree { .. })),
        "reparent into self should fail"
    );

    // The tree is unchanged after the rejected reparents.
    let a_view = model.get_element(a)?;
    ensure_eq!(a_view.owner, Some(root));
    Ok(())
}

// ===========================================================================
// REQ-MM-003: qualified_name_is_derived_never_stored
// ===========================================================================

#[test]
fn qualified_name_is_derived_never_stored() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;
    let b = create_class(&mut model, a, "b")?;

    // qualifiedName is derived from the ownership chain: root/a/b
    let qn = model.qualified_name(b)?;
    ensure_eq!(qn, "root/a/b");

    // It is NOT stored in any column — there is no qualified_name column.
    // Verify by checking the schema: no column named qualified_name.
    let conn = model.connection();
    let cols: Vec<String> = conn
        .prepare("PRAGMA table_info(elements)")?
        .query_map([], |r| r.get::<_, String>(1))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    ensure!(
        !cols.iter().any(|c| c == "qualified_name"),
        "qualified_name must not be stored; found column"
    );

    // Renaming updates the qualified name transparently.
    model.rename_element(a, Some("alpha"))?;
    let qn2 = model.qualified_name(b)?;
    ensure_eq!(qn2, "root/alpha/b");

    // Reparenting also updates it transparently.
    model.reparent_element(b, Some(root))?;
    let qn3 = model.qualified_name(b)?;
    ensure_eq!(qn3, "root/b");
    Ok(())
}

// ===========================================================================
// REQ-MM-005: namespace_add_and_remove_set_and_unset_owner
// ===========================================================================

#[test]
fn namespace_add_and_remove_set_and_unset_owner() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;

    // Adding to a namespace sets owner.
    let a_view = model.get_element(a)?;
    ensure_eq!(a_view.owner, Some(root));

    // Create a class under a.
    let cls = create_class(&mut model, a, "cls")?;
    let cls_view = model.get_element(cls)?;
    ensure_eq!(cls_view.owner, Some(a));

    // Delete the class (removing it from the namespace).
    model.delete_element(cls)?;

    // The element is gone.
    let err = model.get_element(cls).err();
    ensure!(
        matches!(err, Some(Error::ElementNotFound { id }) if id == cls),
        "deleted element should be not found"
    );
    Ok(())
}

// ===========================================================================
// REQ-MM-006: project_root_is_model_package_exactly_one
// ===========================================================================

#[test]
fn project_root_is_model_package_exactly_one() -> TestResult {
    let mut model = fresh_model()?;

    // No root exists yet.
    ensure!(model.root()?.is_none(), "no root before create");

    // Create the root.
    let root_id = new_id();
    let view = model.create_root(root_id, Some("model".to_string()))?;
    ensure_eq!(view.kind, MetaclassKind::Package);
    ensure!(view.owner.is_none(), "root has no owner");

    // The data blob has isModel = true.
    {
        let conn = model.connection();
        let data: String = conn.query_row(
            "SELECT data FROM elements WHERE id = ?1",
            rusqlite::params![root_id.as_uuid().to_string()],
            |r| r.get(0),
        )?;
        ensure!(data.contains("\"isModel\":true"), "data must have isModel");
    }

    // Exactly one root: creating a second is rejected.
    let second = new_id();
    let err = model.create_root(second, Some("second".to_string())).err();
    ensure!(
        matches!(err, Some(Error::RootAlreadyExists { .. })),
        "second root should be rejected"
    );

    // Exactly one element with owner_id IS NULL.
    {
        let conn = model.connection();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM elements WHERE owner_id IS NULL",
            [],
            |r| r.get(0),
        )?;
        ensure_eq!(count, 1);
    }
    Ok(())
}

// ===========================================================================
// REQ-MM-007: packages_nest_to_arbitrary_depth
// ===========================================================================

#[test]
fn packages_nest_to_arbitrary_depth() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;

    // Nest packages 10 levels deep.
    let mut current = root;
    let mut names = vec!["root".to_string()];
    for i in 1..=10 {
        let name = format!("p{i}");
        let pkg = create_pkg(&mut model, current, &name)?;
        current = pkg;
        names.push(name);
    }

    // The deepest package's qualified name is the full chain.
    let qn = model.qualified_name(current)?;
    ensure_eq!(qn, names.join("/"));

    // The closure table reflects the depth.
    let conn = model.connection();
    let ancestors = smith_store::closure::ancestors(conn, &current.as_uuid().to_string())?;
    // 11 nodes: self + 10 ancestors.
    ensure_eq!(ancestors.len(), 11);
    // The root is at depth 10.
    let root_depth = ancestors
        .iter()
        .find(|r| r.id == root.as_uuid().to_string())
        .map(|r| r.depth)
        .ok_or("root not in ancestor chain")?;
    ensure_eq!(root_depth, 10);
    Ok(())
}

// ===========================================================================
// REQ-MM-011: comment_requires_body_and_dies_with_owner
// ===========================================================================

#[test]
fn comment_requires_body_and_dies_with_owner() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let cls = create_class(&mut model, root, "cls")?;

    // Non-empty body is required.
    let cid = new_id();
    let err = model.create_comment(cid, cls, "   ").err();
    ensure!(
        matches!(err, Some(Error::EmptyCommentBody)),
        "empty body should be rejected"
    );

    // A valid comment can be created.
    let cid2 = new_id();
    let view = model.create_comment(cid2, cls, "a note")?;
    ensure_eq!(view.body, "a note");
    ensure_eq!(view.owner, cls);

    // Deleting the owning element deletes the comment (ON DELETE CASCADE).
    // But cls may have no children and no relationship refs, so delete works.
    model.delete_element(cls)?;

    // The comment is gone.
    let err = model.get_comment(cid2).err();
    ensure!(
        matches!(err, Some(Error::ElementNotFound { id }) if id == cid2),
        "comment should be deleted with its owner"
    );
    Ok(())
}

// ===========================================================================
// INV-MM-004: relationship_endpoints_must_exist
// ===========================================================================

#[test]
fn relationship_endpoints_must_exist() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_class(&mut model, root, "a")?;
    let ghost = new_id(); // does not exist

    // Creating a relationship with a non-existent source is rejected.
    let rid = new_id();
    let err = model
        .create_relationship(&CreateRelationship {
            id: rid,
            kind: "Association".to_string(),
            source: ghost,
            target: a,
            owner: root,
        })
        .err();
    ensure!(
        matches!(err, Some(Error::RelationshipEndpointNotFound { endpoint, .. }) if endpoint == ghost),
        "non-existent source should be rejected"
    );

    // Creating a relationship with a non-existent target is rejected.
    let err = model
        .create_relationship(&CreateRelationship {
            id: new_id(),
            kind: "Association".to_string(),
            source: a,
            target: ghost,
            owner: root,
        })
        .err();
    ensure!(
        matches!(err, Some(Error::RelationshipEndpointNotFound { endpoint, .. }) if endpoint == ghost),
        "non-existent target should be rejected"
    );

    // A valid relationship (both endpoints exist) succeeds.
    let rid2 = new_id();
    let view = model.create_relationship(&CreateRelationship {
        id: rid2,
        kind: "Association".to_string(),
        source: a,
        target: a,
        owner: root,
    })?;
    ensure_eq!(view.source, a);
    ensure_eq!(view.target, a);
    Ok(())
}

// ===========================================================================
// INV-MM-005: qualified_name_derivation_terminates
// ===========================================================================

#[test]
fn qualified_name_derivation_terminates() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;
    let b = create_pkg(&mut model, a, "b")?;

    // qualifiedName derivation walks up the ownership chain and terminates.
    let qn = model.qualified_name(b)?;
    ensure_eq!(qn, "root/a/b");

    // After reparenting b to root, the derivation still terminates.
    model.reparent_element(b, Some(root))?;
    let qn2 = model.qualified_name(b)?;
    ensure_eq!(qn2, "root/b");

    // The derivation must not infinite-loop: it returns promptly for any id.
    // (The closure table guarantees termination because the ownership tree
    // is acyclic — INV-MM-001 — and reparent-into-own-subtree is rejected.)
    let qn3 = model.qualified_name(root)?;
    ensure_eq!(qn3, "root");
    Ok(())
}

// ===========================================================================
// REQ-PERS-001: mutation_is_durable_in_sqlite_after_commit
// ===========================================================================

#[test]
fn mutation_is_durable_in_sqlite_after_commit() -> TestResult {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("dur.smith");

    // Create a model, add elements, close it.
    let root_id;
    let cls_id;
    {
        let mut model = Model::open(&path)?;
        root_id = create_root(&mut model, "root")?;
        cls_id = create_class(&mut model, root_id, "cls")?;
    } // model dropped → store checkpointed

    // Reopen: the data must be durable.
    let model2 = Model::open(&path)?;
    let root_view = model2.get_element(root_id)?;
    ensure_eq!(root_view.name.as_deref(), Some("root"));
    let cls_view = model2.get_element(cls_id)?;
    ensure_eq!(cls_view.name.as_deref(), Some("cls"));

    // The closure table is also durable.
    let conn = model2.connection();
    let ancestors = smith_store::closure::ancestors(conn, &cls_id.as_uuid().to_string())?;
    ensure!(
        ancestors
            .iter()
            .any(|r| r.id == root_id.as_uuid().to_string()),
        "root must be an ancestor of cls after reopen"
    );
    Ok(())
}

// ===========================================================================
// REQ-PERS-012: model_holds_single_write_connection
// ===========================================================================

#[test]
fn model_holds_single_write_connection() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;

    // The model exposes exactly one connection reference. All mutations and
    // reads go through it (REQ-PERS-012: single write connection).
    let conn1 = model.connection();
    let conn2 = model.connection();
    ensure!(
        std::ptr::eq(conn1, conn2),
        "model must expose the same connection on every call"
    );

    // The connection is usable: a read works through it.
    let count: i64 = conn1.query_row(
        "SELECT COUNT(*) FROM elements WHERE owner_id IS NULL",
        [],
        |r| r.get(0),
    )?;
    ensure_eq!(count, 1);

    // And a write works through it (create another element).
    let a = create_pkg(&mut model, root, "a")?;
    let a_view = model.get_element(a)?;
    ensure_eq!(a_view.owner, Some(root));
    Ok(())
}

// ===========================================================================
// REQ-PERS-013: failed_transaction_leaves_store_and_state_unchanged
// ===========================================================================

#[test]
fn failed_transaction_leaves_store_and_state_unchanged() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;
    let b = create_class(&mut model, a, "b")?;

    // REQ-PERS-013: a failed mutation must not alter the projection at all.
    // Snapshot the projection node/edge sets before the failing mutations.
    let proj_ids_before = projection_element_ids(model.projection());
    let proj_edges_before = projection_edge_set(model.projection());

    // Attempt a reparent that would form a cycle (a into b, its own descendant).
    // This must fail and leave the store unchanged.
    let err = model.reparent_element(a, Some(b));
    ensure!(err.is_err(), "cyclic reparent must fail");

    // After the failed reparent, a's owner is still root (unchanged).
    let a_view = model.get_element(a)?;
    ensure_eq!(a_view.owner, Some(root));

    // And b's owner is still a (unchanged).
    let b_view = model.get_element(b)?;
    ensure_eq!(b_view.owner, Some(a));

    // The closure table is also unchanged: a's ancestors are [a, root].
    {
        let conn = model.connection();
        let a_ancestors = smith_store::closure::ancestors(conn, &a.as_uuid().to_string())?;
        let ancestor_ids: Vec<String> = a_ancestors.into_iter().map(|r| r.id).collect();
        ensure_eq!(
            ancestor_ids,
            vec![a.as_uuid().to_string(), root.as_uuid().to_string()]
        );
    }

    // REQ-PERS-013: the projection must be unchanged after the failed reparent.
    let proj_ids_after = projection_element_ids(model.projection());
    let proj_edges_after = projection_edge_set(model.projection());
    ensure_eq!(
        proj_ids_after,
        proj_ids_before,
        "projection nodes must be unchanged after failed reparent"
    );
    ensure_eq!(
        proj_edges_after,
        proj_edges_before,
        "projection edges must be unchanged after failed reparent"
    );

    // Attempt to create a relationship referencing a non-existent element.
    // This fails before the transaction starts, so the store is untouched.
    let ghost = new_id();
    let _err = model
        .create_relationship(&CreateRelationship {
            id: new_id(),
            kind: "Association".to_string(),
            source: ghost,
            target: b,
            owner: root,
        })
        .err()
        .ok_or("expected error")?;
    {
        let conn = model.connection();
        let rel_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM relationships", [], |r| r.get(0))?;
        ensure_eq!(
            rel_count,
            0,
            "no relationship should exist after failed create"
        );
    }

    // REQ-PERS-013: the projection must still be unchanged after the failed
    // relationship create.
    let proj_ids_final = projection_element_ids(model.projection());
    let proj_edges_final = projection_edge_set(model.projection());
    ensure_eq!(
        proj_ids_final,
        proj_ids_before,
        "projection nodes must be unchanged after failed relationship create"
    );
    ensure_eq!(
        proj_edges_final,
        proj_edges_before,
        "projection edges must be unchanged after failed relationship create"
    );
    Ok(())
}

// ===========================================================================
// Extra: delete element referenced by relationship is rejected
// (store-level ON DELETE RESTRICT surfaced as API error)
// ===========================================================================

#[test]
fn delete_element_referenced_by_relationship_is_rejected() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_class(&mut model, root, "a")?;
    let b = create_class(&mut model, root, "b")?;

    // Create a relationship a -> b.
    let rid = new_id();
    model.create_relationship(&CreateRelationship {
        id: rid,
        kind: "Association".to_string(),
        source: a,
        target: b,
        owner: root,
    })?;

    // Deleting a (referenced by the relationship as source) must be rejected.
    let err = model.delete_element(a).err();
    ensure!(
        matches!(err, Some(Error::ElementReferencedByRelationship { element, relationship })
            if element == a && relationship == rid),
        "deleting referenced element should be rejected, got {err:?}"
    );

    // Deleting b (referenced as target) must also be rejected.
    let err = model.delete_element(b).err();
    ensure!(
        matches!(err, Some(Error::ElementReferencedByRelationship { .. })),
        "deleting target element should be rejected"
    );

    // After the failed deletes, both elements still exist.
    ensure!(model.get_element(a).is_ok());
    ensure!(model.get_element(b).is_ok());

    // After deleting the relationship, the elements can be deleted.
    model.delete_relationship(rid)?;
    model.delete_element(b)?;
    model.delete_element(a)?;
    model.delete_element(root)?;
    Ok(())
}

// ===========================================================================
// Extra: delete element with children is rejected
// (store-level ON DELETE RESTRICT on owner_id)
// ===========================================================================

#[test]
fn delete_element_with_children_is_rejected() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let a = create_pkg(&mut model, root, "a")?;
    let b = create_class(&mut model, a, "b")?;

    // Deleting a (which owns b) must be rejected.
    let err = model.delete_element(a).err();
    ensure!(
        matches!(err, Some(Error::ElementHasChildren { element, count })
            if element == a && count == 1),
        "deleting element with children should be rejected, got {err:?}"
    );

    // After deleting the child, the parent can be deleted.
    model.delete_element(b)?;
    model.delete_element(a)?;
    Ok(())
}

// ===========================================================================
// Finding 1: create_root is a single transaction setting isModel on insert
// (REQ-PERS-013 / REQ-MM-006): the root row and its isModel flag commit
// together — no intermediate committed state can durably persist a root
// without isModel = true.
// ===========================================================================

#[test]
fn create_root_sets_is_model_in_a_single_transaction() -> TestResult {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let mut model = fresh_model()?;

    // Count commits on the single write connection. create_root must commit
    // exactly once: the element row (with isModel already in its data blob)
    // and the closure rows commit together.
    let commits = Arc::new(AtomicUsize::new(0));
    let hook = {
        let c = Arc::clone(&commits);
        move || {
            c.fetch_add(1, Ordering::SeqCst);
            false // do not roll back
        }
    };
    model.connection().commit_hook(Some(hook))?;

    let root_id = new_id();
    let view = model.create_root(root_id, Some("model".to_string()))?;
    ensure_eq!(view.kind, MetaclassKind::Package);
    ensure!(view.owner.is_none(), "root has no owner");

    // The data blob durably carries isModel = true (no second UPDATE needed).
    let conn = model.connection();
    let data: String = conn.query_row(
        "SELECT data FROM elements WHERE id = ?1",
        rusqlite::params![root_id.as_uuid().to_string()],
        |r| r.get(0),
    )?;
    ensure!(
        data.contains("\"isModel\":true"),
        "root data must carry isModel=true, got {data}"
    );

    // Exactly one commit: the row and the flag are a single transaction.
    let count = commits.load(Ordering::SeqCst);
    ensure_eq!(
        count,
        1,
        "create_root must commit exactly once, got {count}"
    );
    Ok(())
}

// ===========================================================================
// Finding 2: delete_element pre-checks relationships.owner_id (RESTRICT)
// A relationship owned by the deleted element must surface as the typed
// ElementReferencedByRelationship error, not a raw FK violation.
// ===========================================================================

#[test]
fn delete_element_owned_by_relationship_is_rejected() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;
    let pkg = create_pkg(&mut model, root, "pkg")?;
    let cls = create_class(&mut model, root, "cls")?;

    // A self-loop relationship on cls, owned by pkg. Deleting pkg is blocked
    // by relationships.owner_id (neither source_id nor target_id matches pkg).
    let rid = new_id();
    model.create_relationship(&CreateRelationship {
        id: rid,
        kind: "Association".to_string(),
        source: cls,
        target: cls,
        owner: pkg,
    })?;

    // Deleting the relationship's owner (pkg) must surface the typed error.
    let err = model.delete_element(pkg).err();
    ensure!(
        matches!(err, Some(Error::ElementReferencedByRelationship { element, relationship })
            if element == pkg && relationship == rid),
        "deleting relationship owner should be rejected with typed error, got {err:?}"
    );

    // After removing the relationship, the package can be deleted.
    model.delete_relationship(rid)?;
    model.delete_element(pkg)?;
    Ok(())
}

// ===========================================================================
// Finding 3: corrupt store rows surface as typed errors, not fabricated
// values (REQ-ARCH-017/018: fail fast, never swallow). An unparseable id or
// unknown kind in a store row must return Error::CorruptStore.
// ===========================================================================

/// Seed a raw element row directly into the store, bypassing the model API,
/// so a read must interpret the (corrupt) columns.
fn seed_element_row(
    model: &Model,
    id: &str,
    kind: &str,
    owner_id: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = model.connection();
    conn.execute(
        "INSERT INTO elements (id, kind, name, owner_id, visibility, data, created_at, \
         updated_at) VALUES (?1, ?2, NULL, ?3, 'public', '{}', \
         '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
        rusqlite::params![id, kind, owner_id],
    )?;
    Ok(())
}

#[test]
fn corrupt_store_id_surfaces_typed_error() -> TestResult {
    let model = fresh_model()?;

    // Seed the only row: a root (owner_id NULL) whose id is not a valid UUID.
    // A read via root() must interpret the corrupt id column and fail fast.
    seed_element_row(&model, "not-a-valid-uuid", "Class", None)?;

    let err = model.root().err();
    ensure!(
        matches!(err, Some(Error::CorruptStore { field, .. }) if field == "id"),
        "corrupt id should surface as CorruptStore(id), got {err:?}"
    );
    Ok(())
}

#[test]
fn corrupt_store_kind_surfaces_typed_error() -> TestResult {
    let mut model = fresh_model()?;
    let root = create_root(&mut model, "root")?;

    // Seed a valid-id row whose kind is unknown (corruption), owned by root.
    let bad_id = new_id();
    seed_element_row(
        &model,
        &bad_id.as_uuid().to_string(),
        "BogusKind",
        Some(&root.as_uuid().to_string()),
    )?;

    // Reading it must fail fast with CorruptStore naming the kind field.
    let err = model.get_element(bad_id).err();
    ensure!(
        matches!(err, Some(Error::CorruptStore { field, .. }) if field == "kind"),
        "unknown kind should surface as CorruptStore(kind), got {err:?}"
    );
    Ok(())
}
