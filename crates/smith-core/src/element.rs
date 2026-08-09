//! Base element and relationship shapes from the metamodel core.
//!
//! These hold the data the metamodel describes; derived quantities that need
//! the ownership graph (e.g. `qualifiedName`) are a model-layer concern and do
//! not live here. Names are optional and not unique within a namespace
//! (`REQ-MM-004`); identity (the [`ElementId`]) is the only key.

use crate::id::ElementId;
use crate::stereotype::AppliedStereotype;
use crate::visibility::Visibility;

/// A named, owned element: the base shape every specializable element shares.
///
/// `name` is optional and carries no uniqueness constraint (`REQ-MM-004`):
/// siblings may share a name because identity is the [`ElementId`], not the
/// name. Defaults to [`Visibility::Public`] when constructed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedElement {
    /// Immutable identity.
    pub id: ElementId,
    /// Owning namespace, or [`None`] only for the project root.
    pub owner: Option<ElementId>,
    /// Human-readable name; mutable; not unique.
    pub name: Option<String>,
    /// UML visibility (defaults to public).
    pub visibility: Visibility,
    /// Comments attached to (owned by) this element.
    pub comments: Vec<Comment>,
    /// Stereotype applications on this element.
    pub stereotypes: Vec<AppliedStereotype>,
}

impl NamedElement {
    /// A new named element with no owner, no name, and default visibility.
    #[must_use]
    pub fn new(id: ElementId) -> Self {
        Self {
            id,
            owner: None,
            name: None,
            visibility: Visibility::default(),
            comments: Vec::new(),
            stereotypes: Vec::new(),
        }
    }

    /// Sets the owning namespace.
    #[must_use]
    pub fn with_owner(mut self, owner: ElementId) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Sets the (optional) name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Attaches a comment.
    #[must_use]
    pub fn with_comment(mut self, comment: Comment) -> Self {
        self.comments.push(comment);
        self
    }

    /// Applies a stereotype.
    #[must_use]
    pub fn with_stereotype(mut self, stereotype: AppliedStereotype) -> Self {
        self.stereotypes.push(stereotype);
        self
    }
}

/// A comment attached to an element (`REQ-MM-011`).
///
/// The owning element is implicit; `annotated_elements` holds any additional
/// cross-references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// The comment text (Markdown allowed); must be non-empty.
    pub body: String,
    /// Elements this comment additionally annotates (the owner is implicit).
    pub annotated_elements: Vec<ElementId>,
}

/// Error raised when constructing an invalid [`Comment`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CommentError {
    /// The comment body was empty / whitespace-only.
    #[error("comment body must be non-empty")]
    EmptyBody,
}

impl Comment {
    /// Builds a comment, rejecting an empty body (`REQ-MM-011`).
    ///
    /// # Errors
    /// Returns [`CommentError::EmptyBody`] when `body` is empty or whitespace.
    pub fn new(body: impl Into<String>) -> Result<Self, CommentError> {
        let body = body.into();
        if body.trim().is_empty() {
            return Err(CommentError::EmptyBody);
        }
        Ok(Self {
            body,
            annotated_elements: Vec::new(),
        })
    }

    /// Adds an annotated element reference.
    #[must_use]
    pub fn annotate(mut self, element: ElementId) -> Self {
        self.annotated_elements.push(element);
        self
    }
}

/// Base of every edge in the model graph (OMG UML §7.9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    /// Edge identity.
    pub id: ElementId,
    /// Owning namespace (required for an edge).
    pub owner: ElementId,
}

impl Relationship {
    /// Builds an edge owned by `owner`.
    #[must_use]
    pub fn new(id: ElementId, owner: ElementId) -> Self {
        Self { id, owner }
    }
}

/// A directed edge: "from" sources to "to" targets (OMG UML §19.12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectedRelationship {
    /// The underlying [`Relationship`] (identity + owner).
    pub relationship: Relationship,
    /// The "from" elements.
    pub sources: Vec<ElementId>,
    /// The "to" elements.
    pub targets: Vec<ElementId>,
}

impl DirectedRelationship {
    /// A directed edge with no endpoints yet.
    #[must_use]
    pub fn new(relationship: Relationship) -> Self {
        Self {
            relationship,
            sources: Vec::new(),
            targets: Vec::new(),
        }
    }

    /// A binary edge: exactly one source and one target (`REQ-MM-012`).
    #[must_use]
    pub fn binary(relationship: Relationship, source: ElementId, target: ElementId) -> Self {
        Self {
            relationship,
            sources: vec![source],
            targets: vec![target],
        }
    }

    /// Whether this edge is binary (one source, one target).
    #[must_use]
    pub fn is_binary(&self) -> bool {
        self.sources.len() == 1 && self.targets.len() == 1
    }
}
