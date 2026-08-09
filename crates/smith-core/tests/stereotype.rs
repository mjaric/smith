//! `REQ-MM-008` / `INV-MM-003`: reject stereotypes applied to non-extended metaclasses.
//! `REQ-MM-009`: an element carries multiple stereotypes with disjoint tag namespaces.

use smith_core::{
    validate_stereotype_application, AppliedStereotype, ElementId, MetaclassKind, NamedElement,
    Stereotype,
};

#[test]
fn stereotype_application_rejects_non_extended_metaclass() {
    let stereotype = Stereotype::new(ElementId::new(), "entity", &[MetaclassKind::Class]);

    // A metaclass in extendedMetaclasses is accepted.
    let accepted = validate_stereotype_application(&stereotype, MetaclassKind::Class);
    assert!(accepted.is_ok());

    // A metaclass NOT in extendedMetaclasses is rejected (REQ-MM-008 / INV-MM-003).
    let rejected = validate_stereotype_application(&stereotype, MetaclassKind::Requirement);
    assert!(rejected.is_err());
}

#[test]
fn element_carries_multiple_stereotypes_with_disjoint_tag_namespaces() {
    // Two stereotypes, each defining a tag named "id". There is no collision
    // because tag namespaces are per-application (REQ-MM-009).
    let persistence = AppliedStereotype::new(ElementId::new()).with_tag("id", "uuid-aaaa");
    let audit = AppliedStereotype::new(ElementId::new()).with_tag("id", "AUD-0001");

    let element = NamedElement::new(ElementId::new())
        .with_stereotype(persistence)
        .with_stereotype(audit);

    assert_eq!(element.stereotypes.len(), 2);
    // Each application independently keeps its own "id" tag value.
    assert_eq!(element.stereotypes[0].tag_values[0].name, "id");
    assert_eq!(element.stereotypes[0].tag_values[0].value, "uuid-aaaa");
    assert_eq!(element.stereotypes[1].tag_values[0].name, "id");
    assert_eq!(element.stereotypes[1].tag_values[0].value, "AUD-0001");
}
