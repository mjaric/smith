//! `smith-model` acceptance tests for issue #6 — the Model API CRUD surface.
//!
//! One test per acceptance criterion, named per the issue. Style: deterministic
//! tests return [`TestResult`] and use the `ensure` macros — repo lints deny
//! `expect`/`unwrap`/`panic`.

use std::collections::BTreeSet;

use smith_core::{ElementId, MetaclassKind, Visibility};
use smith_model::{CreateElement, CreateRelationship, Error, Model};

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
fn create_root(model: &Model, name: &str) -> Result<ElementId, Error> {
    let id = new_id();
    model.create_root(id, Some(name.to_string()))?;
    Ok(id)
}

/// Create a package under `owner`.
fn create_pkg(model: &Model, owner: ElementId, name: &str) -> Result<ElementId, Error> {
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
fn create_class(model: &Model, owner: ElementId, name: &str) -> Result<ElementId, Error> {
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

// ===========================================================================
// REQ-MM-002: owner tree is acyclic and connected after mutations
// (incl. reparent_into_own_subtree_is_rejected)
// ===========================================================================

#[test]
fn owner_tree_is_acyclic_and_connected_after_mutations() -> TestResult {
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_pkg(&model, root, "a")?;
    let b = create_pkg(&model, a, "b")?;
    let c = create_pkg(&model, b, "c")?;

    // After creating root -> a -> b -> c, every element is reachable from root.
    let conn = model.connection();
    let all_ids: BTreeSet<String> = conn
        .prepare("SELECT id FROM elements")?
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<rusqlite::Result<BTreeSet<String>>>()?;
    ensure_eq!(all_ids.len(), 4);

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
    let c_ancestors = smith_store::closure::ancestors(conn, &c.as_uuid().to_string())?;
    let ancestor_ids: BTreeSet<String> = c_ancestors.into_iter().map(|r| r.id).collect();
    ensure!(
        ancestor_ids.contains(&root.as_uuid().to_string()),
        "root must be an ancestor of c after reparent"
    );
    Ok(())
}

#[test]
fn reparent_into_own_subtree_is_rejected() -> TestResult {
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_pkg(&model, root, "a")?;
    let b = create_pkg(&model, a, "b")?;
    let c = create_pkg(&model, b, "c")?;

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
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_pkg(&model, root, "a")?;
    let b = create_class(&model, a, "b")?;

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
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_pkg(&model, root, "a")?;

    // Adding to a namespace sets owner.
    let a_view = model.get_element(a)?;
    ensure_eq!(a_view.owner, Some(root));

    // Create a class under a.
    let cls = create_class(&model, a, "cls")?;
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
    let model = fresh_model()?;

    // No root exists yet.
    ensure!(model.root()?.is_none(), "no root before create");

    // Create the root.
    let root_id = new_id();
    let view = model.create_root(root_id, Some("model".to_string()))?;
    ensure_eq!(view.kind, MetaclassKind::Package);
    ensure!(view.owner.is_none(), "root has no owner");

    // The data blob has isModel = true.
    let conn = model.connection();
    let data: String = conn.query_row(
        "SELECT data FROM elements WHERE id = ?1",
        rusqlite::params![root_id.as_uuid().to_string()],
        |r| r.get(0),
    )?;
    ensure!(data.contains("\"isModel\":true"), "data must have isModel");

    // Exactly one root: creating a second is rejected.
    let second = new_id();
    let err = model
        .create_root(second, Some("second".to_string()))
        .err();
    ensure!(
        matches!(err, Some(Error::RootAlreadyExists { .. })),
        "second root should be rejected"
    );

    // Exactly one element with owner_id IS NULL.
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM elements WHERE owner_id IS NULL",
        [],
        |r| r.get(0),
    )?;
    ensure_eq!(count, 1);
    Ok(())
}

// ===========================================================================
// REQ-MM-007: packages_nest_to_arbitrary_depth
// ===========================================================================

#[test]
fn packages_nest_to_arbitrary_depth() -> TestResult {
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;

    // Nest packages 10 levels deep.
    let mut current = root;
    let mut names = vec!["root".to_string()];
    for i in 1..=10 {
        let name = format!("p{i}");
        let pkg = create_pkg(&model, current, &name)?;
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
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let cls = create_class(&model, root, "cls")?;

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
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_class(&model, root, "a")?;
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
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_pkg(&model, root, "a")?;
    let b = create_pkg(&model, a, "b")?;

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
        let model = Model::open(&path)?;
        root_id = create_root(&model, "root")?;
        cls_id = create_class(&model, root_id, "cls")?;
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
        ancestors.iter().any(|r| r.id == root_id.as_uuid().to_string()),
        "root must be an ancestor of cls after reopen"
    );
    Ok(())
}

// ===========================================================================
// REQ-PERS-012: model_holds_single_write_connection
// ===========================================================================

#[test]
fn model_holds_single_write_connection() -> TestResult {
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;

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
    let a = create_pkg(&model, root, "a")?;
    let a_view = model.get_element(a)?;
    ensure_eq!(a_view.owner, Some(root));
    Ok(())
}

// ===========================================================================
// REQ-PERS-013: failed_transaction_leaves_store_and_state_unchanged
// ===========================================================================

#[test]
fn failed_transaction_leaves_store_and_state_unchanged() -> TestResult {
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_pkg(&model, root, "a")?;
    let b = create_class(&model, a, "b")?;

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
    let conn = model.connection();
    let a_ancestors = smith_store::closure::ancestors(conn, &a.as_uuid().to_string())?;
    let ancestor_ids: Vec<String> = a_ancestors.into_iter().map(|r| r.id).collect();
    ensure_eq!(ancestor_ids, vec![a.as_uuid().to_string(), root.as_uuid().to_string()]);

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
    let rel_count: i64 = conn.query_row("SELECT COUNT(*) FROM relationships", [], |r| r.get(0))?;
    ensure_eq!(rel_count, 0, "no relationship should exist after failed create");
    Ok(())
}

// ===========================================================================
// Extra: delete element referenced by relationship is rejected
// (store-level ON DELETE RESTRICT surfaced as API error)
// ===========================================================================

#[test]
fn delete_element_referenced_by_relationship_is_rejected() -> TestResult {
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_class(&model, root, "a")?;
    let b = create_class(&model, root, "b")?;

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
    let model = fresh_model()?;
    let root = create_root(&model, "root")?;
    let a = create_pkg(&model, root, "a")?;
    let b = create_class(&model, a, "b")?;

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
