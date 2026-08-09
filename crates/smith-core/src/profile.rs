//! Profiles and the built-in profile registry (`REQ-MM-010`).
//!
//! The three built-in profiles are present in every project, cannot be deleted
//! (`is_builtin == true`), and their stereotypes are available without an
//! explicit profile application. At the smith-core layer "cannot be deleted" is
//! represented by the [`Profile::is_builtin`] flag; the model API enforces it.

use crate::id::ElementId;
use crate::metaclass::MetaclassKind;
use crate::stereotype::Stereotype;

/// Qualified name of the traceability profile.
pub const TRACEABILITY_PROFILE: &str = "smith::traceability";
/// Qualified name of the requirements profile.
pub const REQUIREMENTS_PROFILE: &str = "smith::requirements";
/// Qualified name of the tests profile.
pub const TESTS_PROFILE: &str = "smith::tests";

/// A profile: a named package of stereotype definitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    /// Identity of the profile element.
    pub id: ElementId,
    /// Qualified name, e.g. `smith::traceability`.
    pub qualified_name: String,
    /// Built-in profiles are permanent and always available (`REQ-MM-010`).
    pub is_builtin: bool,
    /// Stereotypes defined by this profile.
    pub stereotypes: Vec<Stereotype>,
}

impl Profile {
    /// Builds a profile.
    #[must_use]
    pub fn new(id: ElementId, qualified_name: impl Into<String>, is_builtin: bool) -> Self {
        Self {
            id,
            qualified_name: qualified_name.into(),
            is_builtin,
            stereotypes: Vec::new(),
        }
    }

    /// Adds a stereotype definition.
    #[must_use]
    pub fn with_stereotype(mut self, stereotype: Stereotype) -> Self {
        self.stereotypes.push(stereotype);
        self
    }
}

/// The trace stereotypes shipped in `smith::traceability`.
///
/// Source: `docs/architecture/04-traceability-relations.md` §Relation taxonomy.
/// Every trace relation is a `Dependency` specialized by one of these.
const TRACE_STEREOTYPES: &[(&str, &[MetaclassKind])] = &[
    ("satisfy", &[MetaclassKind::Dependency]),
    ("verify", &[MetaclassKind::Dependency]),
    ("realize", &[MetaclassKind::Dependency]),
    ("deriveReqt", &[MetaclassKind::Dependency]),
    ("refine", &[MetaclassKind::Dependency]),
    ("trace", &[MetaclassKind::Dependency]),
    ("copy", &[MetaclassKind::Dependency]),
];

/// Builds the built-in `smith::traceability` profile.
fn traceability() -> Profile {
    let mut profile = Profile::new(ElementId::new(), TRACEABILITY_PROFILE, true);
    for (name, extended) in TRACE_STEREOTYPES {
        profile = profile.with_stereotype(Stereotype::new(ElementId::new(), *name, extended));
    }
    profile
}

/// Builds the built-in `smith::requirements` profile.
///
/// `Requirement` and `TestCase` are first-class metaclasses (not stereotypes on
/// `Class`), so this profile defines no stereotypes — see
/// `docs/uml-model/04-requirements-and-tests.md` §Built-in profile.
fn requirements() -> Profile {
    Profile::new(ElementId::new(), REQUIREMENTS_PROFILE, true)
}

/// Builds the built-in `smith::tests` profile.
///
/// The metamodel core names this profile (`REQ-MM-010`) but no doc defines its
/// stereotypes; it is therefore shipped empty pending a spec decision.
fn tests() -> Profile {
    Profile::new(ElementId::new(), TESTS_PROFILE, true)
}

/// Returns the three built-in profiles, present and undeletable in every project.
///
/// Order is fixed: traceability, requirements, tests.
#[must_use]
pub fn builtin_profiles() -> Vec<Profile> {
    vec![traceability(), requirements(), tests()]
}
