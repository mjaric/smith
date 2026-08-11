//! `smith-store` acceptance tests for issue #5 — ownership-closure-table
//! maintenance (`docs/architecture/03-persistence.md` REQ-PERS-007/008).
//!
//! One test per acceptance criterion, named per the issue; two `proptest`
//! cases cover the cross-cutting `INV-MM-001` (the closure table always mirrors
//! `elements.owner_id`).
//!
//! Style: deterministic tests return [`TestResult`] and use the `ensure`
//! macros — repo lints deny `expect`/`unwrap`/`panic`. The `proptest` bodies
//! return `Result<(), TestCaseError>` and use `prop_assert!`/`prop_assert_eq!`.

use std::collections::{BTreeSet, HashMap, HashSet};

use proptest::prelude::*;
use proptest::test_runner::TestCaseError;
use rusqlite::{params, Connection};
use smith_store::closure::{self, ClosureRow};
use smith_store::open;

/// Deterministic test outcome: any helper or `ensure` failure fails via `?`.
type TestResult = Result<(), Box<dyn std::error::Error>>;

const NOW: &str = "2026-01-01T00:00:00Z";

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
                $right,
            )
            .into());
        }
    };
}

/// Wrap any debug-able error as a `proptest` test-case failure.
fn tcerr<E: std::fmt::Debug>(e: E) -> TestCaseError {
    TestCaseError::fail(format!("{e:?}"))
}

// --- element ops backed by the real closure maintenance ---------------------

/// Insert an element owned by `owner` (root when `None`) and maintain its
/// closure — exactly what `smith-model` will do inside one transaction.
fn db_create(conn: &Connection, id: &str, owner: Option<&str>) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO elements (id, kind, owner_id, created_at, updated_at) \
         VALUES (?1, 'Package', ?2, ?3, ?3)",
        params![id, owner, NOW],
    )?;
    closure::insert_on_create(conn, id, owner)
}

/// Reparent `id` to `new_owner` (root when `None`) and maintain its closure.
fn db_reparent(conn: &Connection, id: &str, new_owner: Option<&str>) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE elements SET owner_id = ?1, updated_at = ?2 WHERE id = ?3",
        params![new_owner, NOW, id],
    )?;
    closure::update_on_reparent(conn, id, new_owner)
}

/// Delete an element; the closure table's `ON DELETE CASCADE` drops its rows.
fn db_delete(conn: &Connection, id: &str) -> rusqlite::Result<usize> {
    conn.execute("DELETE FROM elements WHERE id = ?1", params![id])
}

// --- query helpers ----------------------------------------------------------

/// `(id, depth)` pairs in row order — stable for equality checks.
fn pairs(rows: Vec<ClosureRow>) -> Vec<(String, i64)> {
    rows.into_iter().map(|r| (r.id, r.depth)).collect()
}

/// Build an expected `(id, depth)` list from `&str` literals for comparison.
fn want(rows: &[(&str, i64)]) -> Vec<(String, i64)> {
    rows.iter().map(|(id, d)| ((*id).to_string(), *d)).collect()
}

/// Every closure row as a sorted set — the ground truth for comparisons.
fn actual_closure(conn: &Connection) -> rusqlite::Result<BTreeSet<(String, String, i64)>> {
    let mut stmt =
        conn.prepare("SELECT ancestor_id, descendant_id, depth FROM ownership_closure")?;
    let rows = stmt.query_map([], |r| {
        Ok((
            r.get::<_, String>(0)?,
            r.get::<_, String>(1)?,
            r.get::<_, i64>(2)?,
        ))
    })?;
    rows.collect()
}

/// Count rows matching `sql` (no params).
fn count(conn: &Connection, sql: &str) -> rusqlite::Result<i64> {
    conn.query_row(sql, [], |row| row.get(0))
}

// --- in-memory oracle (transitive closure of the owner adjacency list) -------

/// Reference closure computed from an owner map: for every element, its full
/// ancestor chain walked up `owner_id`, each `(ancestor, element, depth)`.
fn expected_closure(owners: &HashMap<String, Option<String>>) -> BTreeSet<(String, String, i64)> {
    let mut out = BTreeSet::new();
    for element in owners.keys() {
        out.insert((element.clone(), element.clone(), 0));
        let mut current = element.clone();
        let mut depth = 0_i64;
        while let Some(owner) = owners.get(&current).cloned().flatten() {
            depth += 1;
            out.insert((owner.clone(), element.clone(), depth));
            current = owner;
        }
    }
    out
}

/// Descendants of `root` (inclusive) reachable by walking owners upward.
fn subtree_of(owners: &HashMap<String, Option<String>>, root: &str) -> HashSet<String> {
    let mut subtree = HashSet::from([root.to_string()]);
    for element in owners.keys() {
        let mut current = owners.get(element).cloned().flatten();
        while let Some(owner) = current {
            if owner == root {
                subtree.insert(element.clone());
                break;
            }
            current = owners.get(&owner).cloned().flatten();
        }
    }
    subtree
}

/// Whether `id` owns no children (may be deleted under `owner_id RESTRICT`).
fn is_leaf(owners: &HashMap<String, Option<String>>, id: &str) -> bool {
    !owners.values().any(|owner| owner.as_deref() == Some(id))
}

// ---------------------------------------------------------------------------
// REQ-PERS-007 (create): ancestor chain + self row, nested, multi-depth
// ---------------------------------------------------------------------------

#[test]
fn closure_create_inserts_ancestor_chain_and_self_row() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir.path().join("p.smith"))?;
    let conn = store.connection();
    // root -> a -> b -> c  (depths 0..3)
    db_create(conn, "root", None)?;
    db_create(conn, "a", Some("root"))?;
    db_create(conn, "b", Some("a"))?;
    db_create(conn, "c", Some("b"))?;

    // c's ancestor chain, nearest-first, includes itself.
    let chain = pairs(closure::ancestors(conn, "c")?);
    ensure_eq!(chain, want(&[("c", 0), ("b", 1), ("a", 2), ("root", 3)]));

    // root's subtree, nearest-first, reaches the deepest leaf.
    let sub = pairs(closure::subtree(conn, "root")?);
    ensure_eq!(sub, want(&[("root", 0), ("a", 1), ("b", 2), ("c", 3)]));

    // depth lookups along the chain and the non-edge reverse direction.
    ensure_eq!(closure::depth(conn, "root", "c")?, Some(3));
    ensure_eq!(closure::depth(conn, "a", "c")?, Some(2));
    ensure_eq!(closure::depth(conn, "c", "c")?, Some(0));
    ensure_eq!(closure::depth(conn, "c", "root")?, None::<i64>);

    // every (ancestor, descendant) pair from the chain is present, 4*5/2.. the
    // full triangle for four nodes = 10 rows (incl. self-links).
    ensure_eq!(count(conn, "SELECT COUNT(*) FROM ownership_closure")?, 10);
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-007 (reparent): cross-product move; old chain rows removed
// ---------------------------------------------------------------------------

#[test]
fn closure_reparent_moves_subtree_rows_via_cross_product() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir.path().join("p.smith"))?;
    let conn = store.connection();
    // tree 1: root1 -> a -> b   ;   tree 2: root2 (isolated)
    db_create(conn, "root1", None)?;
    db_create(conn, "a", Some("root1"))?;
    db_create(conn, "b", Some("a"))?;
    db_create(conn, "root2", None)?;

    // move a (and its child b) from root1 to root2
    db_reparent(conn, "a", Some("root2"))?;

    // a's new chain: only root2 above it (root1 link gone)
    let a_chain = pairs(closure::ancestors(conn, "a")?);
    ensure_eq!(a_chain, want(&[("a", 0), ("root2", 1)]));

    // b inherits root2 at depth 2 (the cross-product new owner x subtree)
    let b_chain = pairs(closure::ancestors(conn, "b")?);
    ensure_eq!(b_chain, want(&[("b", 0), ("a", 1), ("root2", 2)]));

    // root1 no longer owns the moved subtree; root2 does
    ensure_eq!(
        pairs(closure::subtree(conn, "root1")?),
        want(&[("root1", 0)])
    );
    ensure_eq!(
        pairs(closure::subtree(conn, "root2")?),
        want(&[("root2", 0), ("a", 1), ("b", 2)])
    );

    // no closure row still ties the moved subtree to the old root
    let stray = count(
        conn,
        "SELECT COUNT(*) FROM ownership_closure \
         WHERE ancestor_id = 'root1' AND descendant_id IN ('a', 'b')",
    )?;
    ensure_eq!(stray, 0);
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-007 (reparent, deep): proptest — internal subtree rows survive a
// move, and the whole table stays equal to the transitive closure.
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn closure_reparent_deep_subtree_keeps_internal_rows_intact(
        builds in prop::collection::vec(create_op(), 3..16),
        moved in any::<usize>(),
        dest in any::<usize>(),
        dest_root in any::<bool>(),
    ) {
        reparent_internal_intact(&builds, moved, dest, dest_root).map_err(tcerr)?;
    }
}

fn create_op() -> impl Strategy<Value = Option<usize>> {
    (any::<bool>(), any::<usize>()).prop_map(|(root, idx)| if root { None } else { Some(idx) })
}

fn reparent_internal_intact(
    builds: &[Option<usize>],
    moved: usize,
    dest: usize,
    dest_root: bool,
) -> Result<(), TestCaseError> {
    let dir = tempfile::tempdir().map_err(tcerr)?;
    let store = open(&dir.path().join("p.smith")).map_err(tcerr)?;
    let conn = store.connection();
    let mut owners = HashMap::<String, Option<String>>::new();
    let mut order = Vec::new();

    db_create(conn, "n0", None).map_err(tcerr)?;
    owners.insert("n0".to_string(), None);
    order.push("n0".to_string());
    for &owner_idx in builds {
        create_under(&mut owners, &mut order, conn, owner_idx)?;
    }
    prop_assert!(!order.is_empty());

    // pick a non-root element with at least one descendant (a real subtree)
    let target = order
        .get(moved % order.len())
        .filter(|id| id.as_str() != "n0")
        .filter(|id| subtree_of(&owners, id).len() > 1)
        .cloned();
    let Some(target) = target else {
        return Ok(()); // this build has no qualifying deep subtree
    };

    // snapshot the internal closure of the target's subtree (rows wholly inside)
    let sub = subtree_of(&owners, &target);
    let before = actual_closure(conn)
        .map_err(tcerr)?
        .into_iter()
        .filter(|(a, d, _)| sub.contains(a) && sub.contains(d))
        .collect::<BTreeSet<_>>();

    // choose a new owner strictly outside the subtree (no cycle): the generated
    // destination, or root (`None`) when it would land inside the subtree.
    let new_owner = if dest_root {
        None
    } else {
        order
            .get(dest % order.len())
            .filter(|candidate| !sub.contains(*candidate))
            .cloned()
    };

    db_reparent(conn, &target, new_owner.as_deref()).map_err(tcerr)?;
    owners.insert(target.clone(), new_owner);

    // internal subtree rows are byte-for-byte unchanged by the move
    let after = actual_closure(conn)
        .map_err(tcerr)?
        .into_iter()
        .filter(|(a, d, _)| sub.contains(a) && sub.contains(d))
        .collect::<BTreeSet<_>>();
    prop_assert_eq!(before, after, "internal subtree rows changed");

    // and the whole table still equals the transitive closure of owner edges
    prop_assert_eq!(
        actual_closure(conn).map_err(tcerr)?,
        expected_closure(&owners)
    );
    Ok(())
}

fn create_under(
    owners: &mut HashMap<String, Option<String>>,
    order: &mut Vec<String>,
    conn: &Connection,
    owner_idx: Option<usize>,
) -> Result<(), TestCaseError> {
    let id = format!("n{}", order.len());
    let owner = owner_idx
        .filter(|_| !order.is_empty())
        .map(|i| order[i % order.len()].clone());
    db_create(conn, &id, owner.as_deref()).map_err(tcerr)?;
    owners.insert(id.clone(), owner);
    order.push(id);
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-007 (delete): ON DELETE CASCADE drops the subtree's closure rows
// ---------------------------------------------------------------------------

#[test]
fn closure_delete_cascades_rows_for_subtree() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir.path().join("p.smith"))?;
    let conn = store.connection();
    // root -> a -> b -> c
    db_create(conn, "root", None)?;
    db_create(conn, "a", Some("root"))?;
    db_create(conn, "b", Some("a"))?;
    db_create(conn, "c", Some("b"))?;
    ensure_eq!(count(conn, "SELECT COUNT(*) FROM ownership_closure")?, 10);

    // bottom-up (owner_id is RESTRICT): drop the subtree rooted at `a`
    ensure_eq!(db_delete(conn, "c")?, 1);
    ensure_eq!(db_delete(conn, "b")?, 1);
    ensure_eq!(db_delete(conn, "a")?, 1);

    // no closure row mentions any member of the deleted subtree
    let stray = count(
        conn,
        "SELECT COUNT(*) FROM ownership_closure \
         WHERE ancestor_id IN ('a','b','c') OR descendant_id IN ('a','b','c')",
    )?;
    ensure_eq!(stray, 0);

    // root is isolated: only its self-link remains
    ensure_eq!(pairs(closure::subtree(conn, "root")?), want(&[("root", 0)]));
    Ok(())
}

// ---------------------------------------------------------------------------
// REQ-PERS-008: maintenance is atomic with the owner mutation (one txn)
// ---------------------------------------------------------------------------

#[test]
fn closure_maintenance_is_atomic_with_owner_mutation() -> TestResult {
    let dir = tempfile::tempdir()?;
    let store = open(&dir.path().join("p.smith"))?;
    let conn = store.connection();
    db_create(conn, "owner", None)?; // committed owner exists

    // one transaction: insert the element AND maintain closure
    let tx = conn.unchecked_transaction()?;
    tx.execute(
        "INSERT INTO elements (id, kind, owner_id, created_at, updated_at) \
         VALUES ('new', 'Package', 'owner', ?1, ?1)",
        params![NOW],
    )?;
    closure::insert_on_create(&tx, "new", Some("owner"))?;

    // inject a mid-transaction failure AFTER both writes succeeded: a duplicate
    // of the self-row just inserted violates the closure primary key.
    let conflict = tx.execute(
        "INSERT INTO ownership_closure (ancestor_id, descendant_id, depth) \
         VALUES ('new', 'new', 0)",
        [],
    );
    ensure!(
        conflict.is_err(),
        "expected a primary-key conflict mid-transaction"
    );

    // the whole transaction unwinds — element row and closure rows together
    tx.rollback()?;

    ensure_eq!(
        count(conn, "SELECT COUNT(*) FROM elements WHERE id = 'new'")?,
        0
    );
    ensure_eq!(
        count(
            conn,
            "SELECT COUNT(*) FROM ownership_closure WHERE descendant_id = 'new'"
        )?,
        0
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Cross-cutting (INV-MM-001): closure always mirrors elements.owner_id, across
// random create / reparent / delete sequences.
// ---------------------------------------------------------------------------

proptest! {
    #[test]
    fn closure_rows_always_mirror_owner_id(ops in prop::collection::vec(mutation_op(), 1..60)) {
        mirror_property(&ops).map_err(tcerr)?;
    }
}

/// One generated mutation: create (optional owner index), reparent (element +
/// new-owner indices), or delete (element index).
#[derive(Debug, Clone)]
enum Mutation {
    Create(Option<usize>),
    Reparent(usize, Option<usize>),
    Delete(usize),
}

fn mutation_op() -> impl Strategy<Value = Mutation> {
    prop_oneof![
        (any::<bool>(), any::<usize>())
            .prop_map(|(root, idx)| { Mutation::Create(if root { None } else { Some(idx) }) }),
        (any::<usize>(), any::<bool>(), any::<usize>())
            .prop_map(|(a, root, b)| { Mutation::Reparent(a, if root { None } else { Some(b) }) }),
        any::<usize>().prop_map(Mutation::Delete),
    ]
}

fn mirror_property(ops: &[Mutation]) -> Result<(), TestCaseError> {
    let dir = tempfile::tempdir().map_err(tcerr)?;
    let store = open(&dir.path().join("p.smith")).map_err(tcerr)?;
    let conn = store.connection();
    let mut model = Model::new();
    model.create(conn, None)?; // seed a root
    prop_assert_eq!(
        actual_closure(conn).map_err(tcerr)?,
        expected_closure(&model.owners)
    );

    for op in ops {
        apply_mutation(conn, &mut model, op)?;
        prop_assert_eq!(
            actual_closure(conn).map_err(tcerr)?,
            expected_closure(&model.owners),
            "closure diverged from owner edges after {:?}",
            op
        );
    }
    Ok(())
}

/// Mutable model of the ownership forest, kept in lockstep with the database.
struct Model {
    owners: HashMap<String, Option<String>>,
    order: Vec<String>,
    next: usize,
}

impl Model {
    fn new() -> Self {
        Self {
            owners: HashMap::new(),
            order: Vec::new(),
            next: 0,
        }
    }

    fn fresh_id(&mut self) -> String {
        let id = format!("e{}", self.next);
        self.next += 1;
        id
    }

    fn create(&mut self, conn: &Connection, owner_idx: Option<usize>) -> Result<(), TestCaseError> {
        let owner = self.resolve_owner(owner_idx);
        let id = self.fresh_id();
        db_create(conn, &id, owner.as_deref()).map_err(tcerr)?;
        self.owners.insert(id.clone(), owner);
        self.order.push(id);
        Ok(())
    }

    fn resolve_owner(&self, idx: Option<usize>) -> Option<String> {
        idx.filter(|_| !self.order.is_empty())
            .map(|i| self.order[i % self.order.len()].clone())
    }

    fn reparent(
        &mut self,
        conn: &Connection,
        elem: usize,
        owner_idx: Option<usize>,
    ) -> Result<(), TestCaseError> {
        let Some(id) = self.index(elem) else {
            return Ok(());
        };
        let new_owner = self.resolve_owner(owner_idx);
        // skip self-parenting and cycles (cycle prevention is smith-model's job)
        if new_owner.as_deref() == Some(id.as_str())
            || new_owner
                .as_ref()
                .is_some_and(|o| subtree_of(&self.owners, &id).contains(o))
        {
            return Ok(());
        }
        db_reparent(conn, &id, new_owner.as_deref()).map_err(tcerr)?;
        self.owners.insert(id, new_owner);
        Ok(())
    }

    fn delete(&mut self, conn: &Connection, elem: usize) -> Result<(), TestCaseError> {
        let Some(id) = self.index(elem) else {
            return Ok(());
        };
        // owner_id is RESTRICT: only leaves may be deleted
        if !is_leaf(&self.owners, &id) {
            return Ok(());
        }
        db_delete(conn, &id).map_err(tcerr)?;
        self.owners.remove(&id);
        self.order.retain(|e| e != &id);
        Ok(())
    }

    fn index(&self, i: usize) -> Option<String> {
        if self.order.is_empty() {
            None
        } else {
            Some(self.order[i % self.order.len()].clone())
        }
    }
}

fn apply_mutation(
    conn: &Connection,
    model: &mut Model,
    op: &Mutation,
) -> Result<(), TestCaseError> {
    match op {
        Mutation::Create(owner_idx) => model.create(conn, *owner_idx),
        Mutation::Reparent(elem, owner_idx) => model.reparent(conn, *elem, *owner_idx),
        Mutation::Delete(elem) => model.delete(conn, *elem),
    }
}
