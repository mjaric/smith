//! `REQ-MM-004`: names are optional and not unique among siblings.

use smith_core::{ElementId, NamedElement};

#[test]
fn name_is_optional_and_not_unique_among_siblings() {
    let parent = ElementId::new();

    // Names are optional: an element may carry no name.
    let unnamed = NamedElement::new(ElementId::new()).with_owner(parent);
    assert!(unnamed.name.is_none());

    // Two siblings MAY share a name — identity is the id, not the name.
    let a = NamedElement::new(ElementId::new())
        .with_owner(parent)
        .with_name("Payment");
    let b = NamedElement::new(ElementId::new())
        .with_owner(parent)
        .with_name("Payment");

    assert_eq!(a.owner, b.owner);
    assert_eq!(a.name, b.name);
    assert_ne!(a.id, b.id);
}
