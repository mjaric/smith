//! The in-memory petgraph projection (`REQ-PERS-002`, `REQ-PERS-003`).
//!
//! A `petgraph DiGraph<ElementNode, EdgeNode>` that hydrates the full ownership
//! tree and relationship graph from `SQLite` on open. Nodes come from the
//! `elements` table; edges come from `relationships` (directed source → target)
//! and `elements.owner_id` (owner → child ownership edges).
//!
//! The projection is a **cache**, not the source of truth. `SQLite` wins on
//! divergence (`REQ-PERS-002`). It is rebuildable from `SQLite` at any time via
//! [`Projection::rebuild`]; no projection-derived state survives a rebuild
//! (`REQ-PERS-003`).
//!
//! Write-through contract (`REQ-PERS-013`): every applied model mutation updates
//! the projection and `SQLite` in one transaction — they update together or not
//! at all. The [`Model`](crate::Model) methods call the projection update
//! methods after a successful `tx.commit()`, so a store failure leaves the
//! projection untouched. If the transaction fails, the projection is NOT
//! updated (they update together or not at all — `REQ-ARCH-007`).

use std::collections::HashMap;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use smith_core::{ElementId, MetaclassKind, Visibility};

use crate::error::Error;
use crate::model::{kind_from_store, parse_id};
use crate::model::{ElementView, RelationshipView};

/// A node in the projection graph: an element snapshot.
///
/// Carries the same identity/kind/owner/name/visibility fields as
/// [`ElementView`] so the projection is usable for live reads and graph
/// algorithms without a round-trip to `SQLite`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementNode {
    /// Immutable identity.
    pub id: ElementId,
    /// Metaclass kind.
    pub kind: MetaclassKind,
    /// Owning namespace id (`None` only for the project root).
    pub owner: Option<ElementId>,
    /// Human-readable name (optional, not unique).
    pub name: Option<String>,
    /// UML visibility.
    pub visibility: Visibility,
}

/// An edge in the projection graph.
///
/// Ownership edges (owner → child) carry [`EdgeKind::Ownership`]; relationship
/// edges (source → target) carry [`EdgeKind::Relationship`] with the
/// relationship id and kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeNode {
    /// What this edge represents.
    pub kind: EdgeKind,
}

/// What a projection edge represents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeKind {
    /// An ownership edge: the source node owns the target node
    /// (from `elements.owner_id`).
    Ownership,
    /// A relationship edge: `source → target` from the `relationships` table,
    /// carrying the relationship's id and kind string.
    Relationship {
        /// The relationship's element id.
        id: ElementId,
        /// The relationship kind string (e.g. `"Association"`).
        kind: String,
    },
}

/// A snapshot of a projection edge for test comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionEdge {
    /// Source element id.
    pub source: ElementId,
    /// Target element id.
    pub target: ElementId,
    /// Edge kind (ownership or relationship).
    pub kind: EdgeKind,
}

/// The in-memory petgraph projection: a `DiGraph<ElementNode, EdgeNode>`.
///
/// Hydrated from `SQLite` on construction; updated write-through by the
/// [`Model`](crate::Model) on every applied mutation. Rebuildable at any time
/// via [`Projection::rebuild`] — `SQLite` is the source of truth
/// (`REQ-PERS-002`).
#[derive(Debug)]
pub struct Projection {
    graph: DiGraph<ElementNode, EdgeNode>,
    /// Element id → node index, for O(1) node lookup during write-through.
    id_to_node: HashMap<ElementId, NodeIndex>,
}

impl Projection {
    /// Hydrate the projection from the live `SQLite` connection.
    ///
    /// Reads all element rows (nodes) and all relationship + ownership edges,
    /// building the full in-memory graph. Called on model open.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Sqlite`] on store failure, or [`Error::CorruptStore`]
    /// when a row holds an unparseable id or unknown kind.
    pub fn hydrate(conn: &rusqlite::Connection) -> Result<Self, Error> {
        let node_count = conn
            .query_row("SELECT COUNT(*) FROM elements", [], |r| r.get::<_, i64>(0))
            .map_or(0, |n| usize::try_from(n).unwrap_or(0));
        let mut proj = Self {
            graph: DiGraph::with_capacity(node_count, 0),
            id_to_node: HashMap::with_capacity(node_count),
        };
        proj.hydrate_nodes(conn)?;
        proj.hydration_edges(conn)?;
        Ok(proj)
    }

    /// Rebuild the projection from `SQLite`, discarding all current graph state.
    ///
    /// `SQLite` wins on divergence (`REQ-PERS-002`): the rebuilt graph is a fresh
    /// hydration from the store, replacing whatever the projection held. No
    /// projection-derived state survives a rebuild (`REQ-PERS-003`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Sqlite`] on store failure, or [`Error::CorruptStore`]
    /// when a row holds an unparseable id or unknown kind.
    pub fn rebuild(&mut self, conn: &rusqlite::Connection) -> Result<(), Error> {
        let node_count = conn
            .query_row("SELECT COUNT(*) FROM elements", [], |r| r.get::<_, i64>(0))
            .map_or(0, |n| usize::try_from(n).unwrap_or(0));
        self.graph = DiGraph::with_capacity(node_count, 0);
        self.id_to_node.clear();
        self.id_to_node.reserve(node_count);
        self.hydrate_nodes(conn)?;
        self.hydration_edges(conn)?;
        Ok(())
    }

    /// The underlying petgraph reference (for algorithm access).
    #[must_use]
    pub fn graph(&self) -> &DiGraph<ElementNode, EdgeNode> {
        &self.graph
    }

    /// Look up a node by element id.
    #[must_use]
    pub fn node(&self, id: ElementId) -> Option<&ElementNode> {
        self.id_to_node.get(&id).map(|&ix| &self.graph[ix])
    }

    /// The number of nodes (elements) in the projection.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// The number of edges (ownership + relationship) in the projection.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Collect all element nodes, sorted by id for stable comparison in tests.
    #[must_use]
    pub fn elements_sorted(&self) -> Vec<&ElementNode> {
        let mut nodes: Vec<&ElementNode> = self
            .id_to_node
            .values()
            .map(|&ix| &self.graph[ix])
            .collect();
        nodes.sort_by_key(|n| n.id);
        nodes
    }

    /// Collect all edges, sorted for stable comparison in tests.
    #[must_use]
    pub fn edges_sorted(&self) -> Vec<ProjectionEdge> {
        let mut edges: Vec<ProjectionEdge> = self
            .graph
            .edge_references()
            .map(|e| {
                let source_node = &self.graph[e.source()];
                let target_node = &self.graph[e.target()];
                ProjectionEdge {
                    source: source_node.id,
                    target: target_node.id,
                    kind: e.weight().kind.clone(),
                }
            })
            .collect();
        edges.sort_by(|a, b| {
            (a.source, a.target, format!("{a:?}")).cmp(&(b.source, b.target, format!("{b:?}")))
        });
        edges
    }

    /// Whether the ownership tree is acyclic (`INV-MM-001`).
    ///
    /// The ownership subgraph (ownership edges only) must be a forest — no
    /// node owns itself transitively. Because the closure table enforces this
    /// at the store level, a well-formed projection should always be acyclic;
    /// this method lets the round-trip test assert the invariant explicitly.
    #[must_use]
    pub fn ownership_is_acyclic(&self) -> bool {
        let ownership_edges: Vec<(NodeIndex, NodeIndex)> = self
            .graph
            .edge_indices()
            .filter_map(|eix| {
                let edge = self.graph.edge_weight(eix)?;
                if edge.kind == EdgeKind::Ownership {
                    let (s, t) = self.graph.edge_endpoints(eix)?;
                    Some((s, t))
                } else {
                    None
                }
            })
            .collect();
        if ownership_edges.is_empty() {
            return true;
        }
        // Build a subgraph of just ownership edges and check for cycles
        // via petgraph's is_cyclic_directed.
        let mut sub: DiGraph<(), ()> = DiGraph::new();
        let mut node_map: HashMap<NodeIndex, NodeIndex> = HashMap::new();
        for (s, t) in &ownership_edges {
            let s2 = *node_map.entry(*s).or_insert_with(|| sub.add_node(()));
            let t2 = *node_map.entry(*t).or_insert_with(|| sub.add_node(()));
            sub.add_edge(s2, t2, ());
        }
        !petgraph::algo::is_cyclic_directed(&sub)
    }

    /// Whether the ownership tree is connected: every ownership-participating
    /// node is reachable from the single root (`INV-MM-001`).
    ///
    /// A connected ownership tree has exactly one root (a node with no
    /// incoming ownership edge) and every other node reachable from it.
    #[must_use]
    pub fn ownership_is_connected(&self) -> bool {
        // Collect nodes that participate in ownership (have ≥1 ownership edge).
        let mut ownership_nodes: std::collections::HashSet<NodeIndex> =
            std::collections::HashSet::new();
        let mut has_incoming: HashMap<NodeIndex, bool> = HashMap::new();
        for eix in self.graph.edge_indices() {
            if let Some(edge) = self.graph.edge_weight(eix) {
                if edge.kind == EdgeKind::Ownership {
                    if let Some((s, t)) = self.graph.edge_endpoints(eix) {
                        ownership_nodes.insert(s);
                        ownership_nodes.insert(t);
                        has_incoming.insert(t, true);
                    }
                }
            }
        }
        if ownership_nodes.is_empty() {
            // No ownership edges → trivially connected only if ≤1 node total.
            return self.graph.node_count() <= 1;
        }
        // Roots: ownership-participating nodes with no incoming ownership edge.
        let roots: Vec<NodeIndex> = ownership_nodes
            .iter()
            .copied()
            .filter(|n| !has_incoming.get(n).copied().unwrap_or(false))
            .collect();
        if roots.len() != 1 {
            return false;
        }
        // DFS from the single root over ownership edges; every
        // ownership-participating node must be reachable.
        let root = roots[0];
        let mut visited: std::collections::HashSet<NodeIndex> = std::collections::HashSet::new();
        visited.insert(root);
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            for edge in self.graph.edges(node) {
                if edge.weight().kind == EdgeKind::Ownership {
                    let child = edge.target();
                    if visited.insert(child) {
                        stack.push(child);
                    }
                }
            }
        }
        visited == ownership_nodes
    }

    // --- write-through update methods (called by Model after tx.commit()) ---

    /// Add a node for a newly created element, plus its ownership edge if the
    /// element has an owner.
    pub(crate) fn add_element(&mut self, view: &ElementView) {
        let node = ElementNode {
            id: view.id,
            kind: view.kind,
            owner: view.owner,
            name: view.name.clone(),
            visibility: view.visibility,
        };
        let ix = self.graph.add_node(node);
        self.id_to_node.insert(view.id, ix);
        if let Some(owner) = view.owner {
            if let Some(&owner_ix) = self.id_to_node.get(&owner) {
                self.graph.add_edge(
                    owner_ix,
                    ix,
                    EdgeNode {
                        kind: EdgeKind::Ownership,
                    },
                );
            }
        }
    }

    /// Update a node's name after a rename.
    pub(crate) fn rename_element(&mut self, id: ElementId, name: Option<&str>) {
        if let Some(&ix) = self.id_to_node.get(&id) {
            if let Some(node) = self.graph.node_weight_mut(ix) {
                node.name = name.map(str::to_string);
            }
        }
    }

    /// Update ownership edges after a reparent: remove the old incoming
    /// ownership edge and add a new one from the new owner.
    pub(crate) fn reparent_element(&mut self, id: ElementId, new_owner: Option<ElementId>) {
        let Some(&ix) = self.id_to_node.get(&id) else {
            return;
        };
        // Remove old incoming ownership edges to this node. Remove one at a
        // time by re-scanning: `remove_edge` is a swap-remove, so indices
        // collected in a batch go stale after the first removal.
        while let Some(eix) = self
            .graph
            .edges_directed(ix, petgraph::Direction::Incoming)
            .find(|e| e.weight().kind == EdgeKind::Ownership)
            .map(|e| e.id())
        {
            self.graph.remove_edge(eix);
        }
        // Update the node's owner field.
        if let Some(node) = self.graph.node_weight_mut(ix) {
            node.owner = new_owner;
        }
        // Add new ownership edge if there's a new owner.
        if let Some(owner) = new_owner {
            if let Some(&owner_ix) = self.id_to_node.get(&owner) {
                self.graph.add_edge(
                    owner_ix,
                    ix,
                    EdgeNode {
                        kind: EdgeKind::Ownership,
                    },
                );
            }
        }
    }

    /// Remove a node and all its edges after a delete.
    ///
    /// `petgraph::DiGraph::remove_node` is a swap-remove: when the deleted node
    /// is not the last, the last node is moved into the vacated index. We remap
    /// the displaced node's `id → NodeIndex` entry so it still points at the
    /// correct (now-moved) slot — without this, `node(displaced_id)` would
    /// index an empty slot (panic) or, once the freed index is reused, silently
    /// return the wrong element's data.
    pub(crate) fn remove_element(&mut self, id: ElementId) {
        if let Some(ix) = self.id_to_node.remove(&id) {
            self.graph.remove_node(ix);
            // After swap-remove, the node formerly at the last index now
            // lives at `ix` — update its map entry if it moved.
            if let Some(displaced) = self.graph.node_weight(ix) {
                self.id_to_node.insert(displaced.id, ix);
            }
        }
    }

    /// Add a relationship edge after relationship creation.
    pub(crate) fn add_relationship(&mut self, view: &RelationshipView) {
        if let (Some(&source_ix), Some(&target_ix)) = (
            self.id_to_node.get(&view.source),
            self.id_to_node.get(&view.target),
        ) {
            self.graph.add_edge(
                source_ix,
                target_ix,
                EdgeNode {
                    kind: EdgeKind::Relationship {
                        id: view.id,
                        kind: view.kind.clone(),
                    },
                },
            );
        }
    }

    /// Remove a relationship edge after relationship deletion.
    ///
    /// `remove_edge` is a swap-remove; re-scan after each removal so we never
    /// act on a stale edge index.
    pub(crate) fn remove_relationship(&mut self, id: ElementId) {
        while let Some(eix) = self.graph.edge_indices().find(|&eix| {
            self.graph
                .edge_weight(eix)
                .is_some_and(|edge| matches!(&edge.kind, EdgeKind::Relationship { id: rel_id, .. } if *rel_id == id))
        }) {
            self.graph.remove_edge(eix);
        }
    }

    // --- private hydration helpers ---

    fn hydrate_nodes(&mut self, conn: &rusqlite::Connection) -> Result<(), Error> {
        let mut stmt = conn.prepare("SELECT id, kind, owner_id, name, visibility FROM elements")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id = parse_id(row.get_ref(0)?.as_str().map_err(rusqlite::Error::from)?)?;
            let kind = kind_from_store(row.get_ref(1)?.as_str().map_err(rusqlite::Error::from)?)?;
            let owner_str = row.get_ref(2)?.as_str();
            let owner = match owner_str {
                Ok(s) => Some(parse_id(s)?),
                Err(rusqlite::types::FromSqlError::InvalidType) => None,
                Err(e) => return Err(rusqlite::Error::from(e).into()),
            };
            let name: Option<String> = row.get(3)?;
            let visibility =
                visibility_from_store(row.get_ref(4)?.as_str().map_err(rusqlite::Error::from)?);
            let node = ElementNode {
                id,
                kind,
                owner,
                name,
                visibility,
            };
            let ix = self.graph.add_node(node);
            self.id_to_node.insert(id, ix);
        }
        Ok(())
    }

    fn hydration_edges(&mut self, conn: &rusqlite::Connection) -> Result<(), Error> {
        // Ownership edges: iterate the hydrated nodes themselves instead of
        // re-querying elements. Each node already carries its owner_id.
        let ownership_pairs: Vec<(ElementId, NodeIndex)> = self
            .graph
            .node_indices()
            .filter_map(|ix| {
                let node = &self.graph[ix];
                node.owner.map(|owner_id| (owner_id, ix))
            })
            .collect();
        let ownership_count = ownership_pairs.len();
        self.graph.reserve_edges(ownership_count);
        for (owner_id, child_ix) in ownership_pairs {
            if let Some(&owner_ix) = self.id_to_node.get(&owner_id) {
                self.graph.add_edge(
                    owner_ix,
                    child_ix,
                    EdgeNode {
                        kind: EdgeKind::Ownership,
                    },
                );
            }
        }
        // Relationship edges from relationships table (source → target).
        let mut stmt = conn.prepare("SELECT id, kind, source_id, target_id FROM relationships")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let rel_id = parse_id(row.get_ref(0)?.as_str().map_err(rusqlite::Error::from)?)?;
            let kind: String = row.get(1)?;
            let source_id = parse_id(row.get_ref(2)?.as_str().map_err(rusqlite::Error::from)?)?;
            let target_id = parse_id(row.get_ref(3)?.as_str().map_err(rusqlite::Error::from)?)?;
            if let (Some(&source_ix), Some(&target_ix)) = (
                self.id_to_node.get(&source_id),
                self.id_to_node.get(&target_id),
            ) {
                self.graph.add_edge(
                    source_ix,
                    target_ix,
                    EdgeNode {
                        kind: EdgeKind::Relationship { id: rel_id, kind },
                    },
                );
            }
        }
        Ok(())
    }
}

/// Resolve a visibility string from the store into a [`Visibility`].
///
/// The canonical store-string → [`Visibility`] mapping, used by both
/// projection hydration and `model.rs`'s `row_to_element_view`.
pub(crate) fn visibility_from_store(s: &str) -> Visibility {
    match s {
        "private" => Visibility::Private,
        "protected" => Visibility::Protected,
        "package" => Visibility::Package,
        _ => Visibility::Public,
    }
}
