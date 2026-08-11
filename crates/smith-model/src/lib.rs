//! The Smith model API (`smith-model`).
//!
//! CRUD over elements and relationships, with ownership-tree invariants
//! enforced and single-transaction persistence through [`smith_store`].
//! Undo/redo command stack replaying do/undo deltas through the model API.
//!
//! - [`Model`] — the project model: a store plus the write API.
//! - [`command::CommandStack`] — the bounded undo/redo stack.
//! - [`command::Command`] / [`command::Actor`] — the command pattern.
//! - [`ElementView`] / [`RelationshipView`] / [`CommentView`] — read snapshots.
//! - [`CreateElement`] / [`CreateRelationship`] — creation requests.
//! - [`error::Error`] — typed errors; no panic crosses the boundary.
//!
//! Non-goals of this crate: the petgraph projection, FTS5 search, and
//! diagrams CRUD.

pub mod command;
pub mod error;
pub mod model;

pub use error::Error;
pub use model::{
    CommentView, CreateElement, CreateRelationship, ElementView, Model, RelationshipView,
};

// Re-export core domain types callers need to construct requests.
pub use smith_core::{
    Comment, DirectedRelationship, ElementId, MetaclassKind, Relationship, Visibility,
};
