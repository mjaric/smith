//! Ownership-closure-table maintenance (`REQ-PERS-007`, `REQ-PERS-008`).
//!
//! The `ownership_closure` table holds the transitive closure of the ownership
//! tree (`elements.owner_id`): one row `(ancestor, descendant, depth)` per
//! reachable ancestor, with `depth = 0` for the self-link (`docs/architecture/
//! 03-persistence.md` §Closure-table maintenance; `docs/research/03-graph-
//! database-options.md` §5). Subtree and ancestor queries are then single
//! indexed joins — the navigator and RTM hot paths.
//!
//! Maintenance is **set-based** and runs in the caller's transaction so it is
//! atomic with the owning-edge mutation (`REQ-PERS-008`): a failure rolls back
//! the element/owner change and the closure change together. The functions here
//! take a [`Connection`] (a [`rusqlite::Transaction`] derefs to one, so pass
//! `&tx` to run inside a transaction); they perform no transaction management
//! of their own.
//!
//! - [`insert_on_create`] — element creation: append the new node beneath its
//!   owner's ancestor chain plus the self-row.
//! - [`update_on_reparent`] — reparent: drop the moved subtree's old ancestor
//!   links, then insert the cross-product of the new owner's chain × the
//!   subtree (two set-based statements, research/03 §5).
//! - Delete is handled declaratively: both closure foreign keys are
//!   `ON DELETE CASCADE`, so removing an element drops every closure row that
//!   names it. No function is needed.
//!
//! Read helpers — [`subtree`], [`ancestors`], [`depth`] — answer "what does
//! this element own transitively", "who owns this element transitively", and
//! "how far apart are these two".
//!
//! This module does **not** validate reparent legality: cycle detection lives
//! one layer up in `smith-model`. A reparent that would create a cycle fails
//! here on a primary-key conflict and rolls back.

use rusqlite::{Connection, OptionalExtension};

/// One row of a closure read: the *other* endpoint's id and its distance.
///
/// For [`subtree`] `id` is a descendant of the queried element; for
/// [`ancestors`] `id` is an ancestor. Both include the self-link (`depth = 0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureRow {
    /// The other endpoint's element id.
    pub id: String,
    /// Edge count between the queried element and [`ClosureRow::id`]
    /// (`0` = the element itself).
    pub depth: i64,
}

/// Maintain the closure table after inserting an element (`REQ-PERS-007`).
///
/// Inserts `(anc, new, depth+1)` for every `(anc, owner, depth)` row in the
/// owner's ancestor chain, plus the self-row `(new, new, 0)`. A `None` owner
/// (a root element) inserts only the self-row.
///
/// Run this in the same transaction as the `elements` insert (`REQ-PERS-008`).
///
/// # Errors
///
/// Returns the underlying [`rusqlite::Error`] when the `INSERT` fails.
pub fn insert_on_create(
    conn: &Connection,
    new_id: &str,
    owner: Option<&str>,
) -> rusqlite::Result<()> {
    // The new node descends from every ancestor of its owner (the owner's own
    // ancestor chain) one level deeper, plus itself at depth 0. A `None` owner
    // (a root) matches nothing in the SELECT, so only the self-row lands.
    conn.execute(
        "INSERT INTO ownership_closure (ancestor_id, descendant_id, depth) \
         SELECT ancestor_id, ?1, depth + 1 \
           FROM ownership_closure \
          WHERE descendant_id = ?2 \
         UNION ALL \
         SELECT ?1, ?1, 0",
        rusqlite::params![new_id, owner],
    )?;
    Ok(())
}

/// Maintain the closure table after reparenting an element (`REQ-PERS-007`).
///
/// Drops every closure row that ties the moved subtree to the old owner's
/// chain, then inserts the cross-product of the new owner's ancestor chain ×
/// the subtree (two set-based statements — `docs/research/03-graph-database-
/// options.md` §5). Internal subtree rows (the self-link and links within the
/// subtree) are preserved; only the external ancestor links move. A `None` new
/// owner (move to root) strips all external links and inserts none.
///
/// Run this in the same transaction as the `elements.owner_id` update
/// (`REQ-PERS-008`). A reparent that would form a cycle violates the closure
/// primary key and rolls back; cycle prevention is `smith-model`'s job.
///
/// # Errors
///
/// Returns the underlying [`rusqlite::Error`] when the `DELETE` or `INSERT`
/// fails.
pub fn update_on_reparent(
    conn: &Connection,
    element_id: &str,
    new_owner: Option<&str>,
) -> rusqlite::Result<()> {
    // (1) Detach the moved subtree from the element's old ancestors: for every
    // descendant of the element, drop rows whose ancestor is a *strict*
    // ancestor of the element (the old ownership chain above it). Internal
    // subtree rows — self-links and links within the subtree — are kept: their
    // ancestor lies inside the subtree, never above it.
    conn.execute(
        "DELETE FROM ownership_closure \
         WHERE descendant_id IN \
               (SELECT descendant_id FROM ownership_closure WHERE ancestor_id = ?1) \
           AND ancestor_id IN \
               (SELECT ancestor_id FROM ownership_closure \
                 WHERE descendant_id = ?1 AND ancestor_id != ?1)",
        rusqlite::params![element_id],
    )?;
    // (2) Re-attach under the new owner: cross-product of the new owner's
    // ancestor chain (the owner and all of its ancestors) × the moved subtree
    // (the element and all of its descendants), depth = chain depth + 1 +
    // subtree depth. A `None` new owner matches nothing, leaving the subtree
    // parented at the root (only its internal rows remain).
    conn.execute(
        "INSERT INTO ownership_closure (ancestor_id, descendant_id, depth) \
         SELECT a.ancestor_id, d.descendant_id, a.depth + 1 + d.depth \
           FROM ownership_closure AS a \
           CROSS JOIN ownership_closure AS d \
          WHERE a.descendant_id = ?2 \
            AND d.ancestor_id = ?1",
        rusqlite::params![element_id, new_owner],
    )?;
    Ok(())
}

/// Descendants of `element_id` ordered nearest-first, including itself.
///
/// One [`ClosureRow`] per descendant; `depth` is the edge count (`0` = the
/// element itself). Filter `depth > 0` for strict descendants.
///
/// # Errors
///
/// Returns the underlying [`rusqlite::Error`] when the query fails.
pub fn subtree(conn: &Connection, element_id: &str) -> rusqlite::Result<Vec<ClosureRow>> {
    let mut stmt = conn.prepare(
        "SELECT descendant_id, depth FROM ownership_closure \
          WHERE ancestor_id = ?1 \
          ORDER BY depth, descendant_id",
    )?;
    let rows = stmt.query_map([element_id], |row| {
        Ok(ClosureRow {
            id: row.get(0)?,
            depth: row.get(1)?,
        })
    })?;
    rows.collect()
}

/// Ancestors of `element_id` ordered nearest-first, including itself.
///
/// One [`ClosureRow`] per ancestor; `depth` is the edge count (`0` = the
/// element itself, `1` = the direct owner, …). Filter `depth > 0` for strict
/// ancestors (the ownership path up to the root).
///
/// # Errors
///
/// Returns the underlying [`rusqlite::Error`] when the query fails.
pub fn ancestors(conn: &Connection, element_id: &str) -> rusqlite::Result<Vec<ClosureRow>> {
    let mut stmt = conn.prepare(
        "SELECT ancestor_id, depth FROM ownership_closure \
          WHERE descendant_id = ?1 \
          ORDER BY depth, ancestor_id",
    )?;
    let rows = stmt.query_map([element_id], |row| {
        Ok(ClosureRow {
            id: row.get(0)?,
            depth: row.get(1)?,
        })
    })?;
    rows.collect()
}

/// The edge count from `ancestor_id` down to `descendant_id`, or `None` if the
/// former is not a (transitive) ancestor of the latter.
///
/// # Errors
///
/// Returns the underlying [`rusqlite::Error`] when the query fails.
pub fn depth(
    conn: &Connection,
    ancestor_id: &str,
    descendant_id: &str,
) -> rusqlite::Result<Option<i64>> {
    conn.query_row(
        "SELECT depth FROM ownership_closure \
          WHERE ancestor_id = ?1 AND descendant_id = ?2",
        rusqlite::params![ancestor_id, descendant_id],
        |row| row.get::<_, i64>(0),
    )
    .optional()
}
