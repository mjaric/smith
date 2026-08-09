//! Stereotypes, their applications, and application validation.
//!
//! A stereotype declares which metaclasses it may extend
//! (`extended_metaclasses`). Applying it to an element whose metaclass is not
//! extended is rejected (`REQ-MM-008`, `INV-MM-003`). An element may carry
//! multiple stereotypes; tag-value namespaces are per-application, so two
//! stereotypes can each define a tag of the same name without colliding
//! (`REQ-MM-009`).

use crate::id::ElementId;
use crate::metaclass::MetaclassKind;

/// A stereotype definition: the metaclasses it extends and its identity.
///
/// Stereotypes are themselves elements (they own an [`ElementId`]); the
/// `extended_metaclasses` list constrains which element kinds they may be
/// applied to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stereotype {
    /// Identity of the stereotype element.
    pub id: ElementId,
    /// Stereotype name (rendered in `«guillemets»`).
    pub name: String,
    /// Metaclasses this stereotype may extend.
    pub extended_metaclasses: Vec<MetaclassKind>,
}

impl Stereotype {
    /// Builds a stereotype extending the given metaclasses.
    #[must_use]
    pub fn new(id: ElementId, name: impl Into<String>, extended: &[MetaclassKind]) -> Self {
        Self {
            id,
            name: name.into(),
            extended_metaclasses: extended.to_vec(),
        }
    }
}

/// A single tagged value on a stereotype application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagValue {
    /// Tag name (scoped to the enclosing application).
    pub name: String,
    /// Tag value (string form for v1).
    pub value: String,
}

impl TagValue {
    /// Builds a tagged value.
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// An application of a stereotype to a specific element.
///
/// `tag_values` are scoped to this application: two applications of different
/// stereotypes each carry their own namespace (`REQ-MM-009`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedStereotype {
    /// The stereotype being applied.
    pub stereotype_id: ElementId,
    /// Values for this application's tags (per-application namespace).
    pub tag_values: Vec<TagValue>,
}

impl AppliedStereotype {
    /// An application of `stereotype_id` with no tag values yet.
    #[must_use]
    pub fn new(stereotype_id: ElementId) -> Self {
        Self {
            stereotype_id,
            tag_values: Vec::new(),
        }
    }

    /// Sets a tag value on this application.
    #[must_use]
    pub fn with_tag(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.tag_values.push(TagValue::new(name, value));
        self
    }
}

/// Error raised when validating a stereotype application.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum StereotypeError {
    /// The element's metaclass is not in the stereotype's `extended_metaclasses`.
    #[error("stereotype {stereotype:?} does not extend metaclass {kind}", kind = element_kind.as_str())]
    MetaclassNotExtended {
        /// Name of the stereotype whose extension was violated.
        stereotype: String,
        /// The metaclass the application targeted.
        element_kind: MetaclassKind,
    },
}

/// Validates that `stereotype` may be applied to an element of `element_kind`.
///
/// This is the `INV-MM-003` / `REQ-MM-008` check: the element's metaclass must
/// appear in the stereotype's `extended_metaclasses`.
///
/// # Errors
/// Returns [`StereotypeError::MetaclassNotExtended`] when `element_kind` is not
/// among `stereotype.extended_metaclasses`.
pub fn validate_stereotype_application(
    stereotype: &Stereotype,
    element_kind: MetaclassKind,
) -> Result<(), StereotypeError> {
    if stereotype.extended_metaclasses.contains(&element_kind) {
        Ok(())
    } else {
        Err(StereotypeError::MetaclassNotExtended {
            stereotype: stereotype.name.clone(),
            element_kind,
        })
    }
}
