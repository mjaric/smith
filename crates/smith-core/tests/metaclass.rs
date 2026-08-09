//! `REQ-MM-015`: the metaclass kind registry is closed.

use std::collections::HashSet;

use smith_core::{MetaclassKind, MetaclassScope};

#[test]
fn metaclass_kind_registry_is_closed() {
    // Every registered kind round-trips id <-> kind, and no two kinds share an id.
    let mut ids = HashSet::new();
    let mut top_level = 0usize;
    let mut sub_element = 0usize;
    for kind in MetaclassKind::ALL {
        let id = kind.as_str();
        assert!(ids.insert(id), "duplicate id {id}");
        assert_eq!(MetaclassKind::from_id(id), Some(*kind));
        match kind.scope() {
            MetaclassScope::TopLevel => top_level += 1,
            MetaclassScope::SubElement => sub_element += 1,
        }
    }

    // Counts match the two spec tables (22 top-level + 37 sub-element = 59).
    assert_eq!(top_level, 22);
    assert_eq!(sub_element, 37);
    assert_eq!(MetaclassKind::ALL.len(), 59);

    // An unregistered id resolves to nothing — that is what "closed" means.
    assert_eq!(MetaclassKind::from_id("NotARealKind"), None);

    // Top-level kinds are the ones a stereotype's extendedMetaclasses references.
    assert_eq!(MetaclassKind::Dependency.scope(), MetaclassScope::TopLevel);
    assert_eq!(MetaclassKind::Comment.scope(), MetaclassScope::TopLevel);
    // Sub-element kinds are tracked but not independently ownable.
    assert_eq!(MetaclassKind::Port.scope(), MetaclassScope::SubElement);
}
