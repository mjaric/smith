//! Pure-Rust domain types for Smith's UML metamodel (Slice 1).
//!
//! Defines element identity, the closed metaclass registry, visibility,
//! multiplicity, and the base element/relationship/stereotype shapes plus
//! stereotype-application validation. No IO and no Smith-crate dependencies
//! (`docs/architecture/01-system-architecture.md` REQ-ARCH-005/006). The
//! normative contract lives in `docs/uml-model/01-metamodel-core.md`.

pub mod element;
pub mod id;
pub mod metaclass;
pub mod multiplicity;
pub mod profile;
pub mod stereotype;
pub mod visibility;

pub use element::{Comment, CommentError, DirectedRelationship, NamedElement, Relationship};
pub use id::ElementId;
pub use metaclass::{MetaclassKind, MetaclassScope};
pub use multiplicity::{Multiplicity, MultiplicityError};
pub use profile::{builtin_profiles, Profile};
pub use stereotype::{
    validate_stereotype_application, AppliedStereotype, Stereotype, StereotypeError, TagValue,
};
pub use visibility::Visibility;
