//! The Smith model API (`smith-model`).
//!
//! CRUD over elements and relationships, with ownership-tree invariants
//! enforced and single-transaction persistence through [`smith_store`].
//!
//! - [`Model`] — the project model: a store plus the write API.
//! - [`ElementView`] / [`RelationshipView`] / [`CommentView`] — read snapshots.
//! - [`CreateElement`] / [`CreateRelationship`] — creation requests.
//! - [`error::Error`] — typed errors; no panic crosses the boundary.
//!
//! Non-goals of this crate: undo/redo command stack, the petgraph projection,
//! FTS5 search, and diagrams CRUD.

pub mod error;
pub mod model;

pub use error::Error;
pub use model::{
    CommentView, CreateElement, CreateRelationship, ElementView, Model, RelationshipView,
};

// Re-export core domain types callers need to construct requests.
pub use smith_core::{Comment, DirectedRelationship, ElementId, MetaclassKind, Relationship,
    Visibility};
