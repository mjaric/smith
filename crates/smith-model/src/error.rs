//! Typed errors for the model API (`REQ-ARCH-017`, `REQ-ARCH-018`).
//!
//! Every fallible operation returns [`Result`]; no panic crosses the model
//! API boundary. Variants carry enough context for a user-facing message and
//! a machine-handleable kind.
//!
//! `ElementId` does not implement `Display` (it is an opaque newtype in
//! `smith-core`); error messages format ids via `Debug` (`{id:?}`), which
//! includes the underlying `UUIDv7`.

use std::path::PathBuf;

use smith_core::ElementId;

/// Every error the model API can raise.
///
/// Variants are typed (not stringly) so callers can match on the kind and
/// surface a clear, actionable message. Store errors are wrapped via the
/// [`Error::Store`] variant; invariant violations are first-class.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The project root was already created (`REQ-MM-006`: exactly one per
    /// project). Creating a second root is rejected.
    #[error("project root already exists (id {existing:?}); a project has exactly one root package")]
    RootAlreadyExists {
        /// The id of the existing root package.
        existing: ElementId,
    },

    /// An element was not found by id.
    #[error("element {id:?} not found")]
    ElementNotFound {
        /// The id that was looked up.
        id: ElementId,
    },

    /// A relationship was not found by id.
    #[error("relationship {id:?} not found")]
    RelationshipNotFound {
        /// The id that was looked up.
        id: ElementId,
    },

    /// A metaclass was supplied that is not a top-level ownable kind
    /// (`REQ-MM-015`).
    #[error("metaclass {kind} is not a top-level ownable element kind")]
    NotTopLevelKind {
        /// The stable string identifier of the rejected kind.
        kind: String,
    },

    /// A relationship source or target does not reference an existing element
    /// (`INV-MM-004`).
    #[error("relationship {kind} endpoint {endpoint:?} does not reference an existing element")]
    RelationshipEndpointNotFound {
        /// The relationship kind string.
        kind: String,
        /// The id of the missing endpoint.
        endpoint: ElementId,
    },

    /// A reparent would move an element into its own subtree, forming a cycle
    /// (`INV-MM-001`).
    #[error("cannot reparent {element:?} into {new_owner:?}: it is in the element's own subtree")]
    ReparentIntoOwnSubtree {
        /// The element being reparented.
        element: ElementId,
        /// The would-be new owner.
        new_owner: ElementId,
    },

    /// Deleting an element referenced by a relationship was rejected
    /// (store-level `ON DELETE RESTRICT`).
    #[error("cannot delete {element:?}: it is referenced by relationship {relationship:?}")]
    ElementReferencedByRelationship {
        /// The element being deleted.
        element: ElementId,
        /// The relationship that references it.
        relationship: ElementId,
    },

    /// Deleting an element that owns children was rejected
    /// (store-level `ON DELETE RESTRICT` on `elements.owner_id`).
    #[error("cannot delete {element:?}: it owns {count} child element(s); delete them first")]
    ElementHasChildren {
        /// The element being deleted.
        element: ElementId,
        /// Number of children blocking deletion.
        count: usize,
    },

    /// A comment body was empty (`REQ-MM-011`).
    #[error("comment body must be non-empty")]
    EmptyCommentBody,
    /// A store-level error (`SQLite` IO, migration, checkpoint).
    #[error("store error on {path}: {detail}")]
    Store {
        /// The store file path.
        path: PathBuf,
        /// The underlying store error.
        #[source]
        detail: smith_store::Error,
    },

    /// A raw `SQLite` error from a model-layer query.
    #[error("sqlite error: {detail}")]
    Sqlite {
        /// The underlying rusqlite error.
        #[source]
        detail: rusqlite::Error,
    },
}

impl Error {
    /// Whether this error is the `ElementNotFound` variant.
    #[must_use]
    pub fn is_element_not_found(&self) -> bool {
        matches!(self, Self::ElementNotFound { .. })
    }
}

impl From<rusqlite::Error> for Error {
    fn from(detail: rusqlite::Error) -> Self {
        Self::Sqlite { detail }
    }
}
