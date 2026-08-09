//! `REQ-MM-012`: directed relationships carry binary sources and targets.

use smith_core::{DirectedRelationship, ElementId, Relationship};

#[test]
fn directed_relationship_binary_sources_and_targets() {
    let owner = ElementId::new();
    let source = ElementId::new();
    let target = ElementId::new();
    let relationship = Relationship::new(ElementId::new(), owner);

    let edge = DirectedRelationship::binary(relationship, source, target);

    assert!(edge.is_binary());
    assert_eq!(edge.sources, vec![source]);
    assert_eq!(edge.targets, vec![target]);

    // N-ary / empty endpoints are not binary (the metamodel allows n-ary; the
    // shape reports non-binary honestly).
    let mut nary = DirectedRelationship::new(Relationship::new(ElementId::new(), owner));
    nary.sources.push(source);
    nary.targets.push(target);
    nary.targets.push(ElementId::new());
    assert!(!nary.is_binary());
}
