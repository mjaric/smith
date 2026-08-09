//! Closed metaclass kind registry (`REQ-MM-015`).
//!
//! The registry is the union of the two tables in
//! `docs/uml-model/01-metamodel-core.md` §Metaclass kind registry:
//!
//! - **Top-level** kinds — independently ownable in the ownership tree. These
//!   are the kinds a stereotype's `extendedMetaclasses` may reference.
//! - **Sub-element / relationship** kinds — belong to a parent element or
//!   diagram, not independently ownable, but each carries a stable string
//!   identifier for view references, search, and constraint checks.
//!
//! Each variant's stable identifier is its own `PascalCase` name (the spec row
//! name verbatim). The set is closed for v1; user extension is via stereotypes,
//! not new metaclasses.

/// Whether a [`MetaclassKind`] is independently ownable or a nested sub-kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaclassScope {
    /// Top-level element kind (ownable in the ownership tree).
    TopLevel,
    /// Sub-element or relationship kind (owned by a parent element / diagram).
    SubElement,
}

/// Declares the closed [`MetaclassKind`] registry from a list of
/// `(variant, scope)` rows. Each variant's stable identifier is its own name
/// (the spec row name), derived with [`stringify!`]. Centralising the rows here
/// keeps the enum, the exhaustive `ALL` slice, and the `as_str` / `scope` /
/// `from_id` accessors in lockstep.
macro_rules! kind_registry {
    ( $( $variant:ident @ $scope:ident ),+ $(,)? ) => {
        /// Every metaclass (element kind) Smith recognises.
        ///
        /// Variants are grouped by scope (top-level first, then sub-element /
        /// relationship kinds); see [`MetaclassScope`].
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MetaclassKind {
            $( $variant ),+
        }

        impl MetaclassKind {
            /// Every kind in the registry, top-level kinds first.
            pub const ALL: &'static [MetaclassKind] = &[ $( MetaclassKind::$variant ),+ ];
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self { $( MetaclassKind::$variant => stringify!($variant) ),+ }
            }

            /// Whether this kind is ownable or a nested sub-kind.
            #[must_use]
            pub const fn scope(self) -> MetaclassScope {
                match self { $( MetaclassKind::$variant => MetaclassScope::$scope ),+ }
            }

            /// Resolve a kind by its stable identifier.
            ///
            /// Returns [`None`] for any string that is not a registered kind,
            /// which is what makes the registry closed.
            #[must_use]
            pub fn from_id(id: &str) -> Option<MetaclassKind> {
                match id {
                    $( stringify!($variant) => Some(MetaclassKind::$variant) ),+,
                    _ => None,
                }
            }
        }
    };
}

kind_registry! {
    // --- Top-level element kinds (ownable; the first spec table) ---
    Package @ TopLevel,
    Class @ TopLevel,
    Interface @ TopLevel,
    DataType @ TopLevel,
    Enumeration @ TopLevel,
    Association @ TopLevel,
    Generalization @ TopLevel,
    Realization @ TopLevel,
    Dependency @ TopLevel,
    UseCase @ TopLevel,
    Actor @ TopLevel,
    Activity @ TopLevel,
    Interaction @ TopLevel,
    StateMachine @ TopLevel,
    Component @ TopLevel,
    Node @ TopLevel,
    Artifact @ TopLevel,
    Requirement @ TopLevel,
    TestCase @ TopLevel,
    Profile @ TopLevel,
    Stereotype @ TopLevel,
    Comment @ TopLevel,
    // --- Sub-element / relationship kinds (the second spec table) ---
    PrimitiveType @ SubElement,
    AssociationClass @ SubElement,
    Signal @ SubElement,
    InstanceSpecification @ SubElement,
    Slot @ SubElement,
    Link @ SubElement,
    Port @ SubElement,
    Connector @ SubElement,
    Part @ SubElement,
    Collaboration @ SubElement,
    CollaborationUse @ SubElement,
    Message @ SubElement,
    Lifeline @ SubElement,
    ExecutionSpecification @ SubElement,
    CombinedFragment @ SubElement,
    InteractionUse @ SubElement,
    Gate @ SubElement,
    State @ SubElement,
    Region @ SubElement,
    Transition @ SubElement,
    Pseudostate @ SubElement,
    FinalState @ SubElement,
    Action @ SubElement,
    ControlNode @ SubElement,
    ObjectNode @ SubElement,
    ActivityPartition @ SubElement,
    Include @ SubElement,
    Extend @ SubElement,
    ExtensionPoint @ SubElement,
    Deployment @ SubElement,
    Manifestation @ SubElement,
    CommunicationPath @ SubElement,
    PackageImport @ SubElement,
    PackageMerge @ SubElement,
    ElementImport @ SubElement,
    Metaclass @ SubElement,
    Extension @ SubElement,
}
