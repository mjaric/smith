//! The Smith model API (`smith-model`).
//!
//! CRUD over elements and relationships, with ownership-tree invariants
//! enforced and single-transaction persistence through [`smith_store`]. The
//! in-memory `petgraph` projection ([`projection::Projection`]) hydrates the
//! full ownership tree + relationship graph from `SQLite` on open and is updated
//! write-through on every applied mutation (`REQ-PERS-002`, `REQ-PERS-003`).
//! Undo/redo command stack replaying do/undo deltas through the model API.
//!
//! - [`Model`] — the project model: a store, the projection, and the write API.
//! - [`projection::Projection`] — the rebuildable petgraph cache.
//! - [`projection::ElementNode`] / [`projection::EdgeNode`] — graph node/edge types.
//! - [`command::CommandStack`] — the bounded undo/redo stack.
//! - [`command::Command`] / [`command::Actor`] — the command pattern.
//! - [`ElementView`] / [`RelationshipView`] / [`CommentView`] — read snapshots.
//! - [`CreateElement`] / [`CreateRelationship`] — creation requests.
//! - [`error::Error`] — typed errors; no panic crosses the boundary.
//!
//! Non-goals of this crate: FTS5 search, diagrams CRUD.

pub mod command;
pub mod error;
pub mod model;
pub mod projection;

pub use error::Error;
pub use model::{
    CommentView, CreateElement, CreateRelationship, ElementView, Model, RelationshipView,
};

// Re-export core domain types callers need to construct requests.
pub use smith_core::{
    Comment, DirectedRelationship, ElementId, MetaclassKind, Relationship, Visibility,
};
// Re-export projection types for callers that need graph access.
pub use projection::{EdgeKind, EdgeNode, ElementNode, Projection, ProjectionEdge};
