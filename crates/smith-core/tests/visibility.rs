//! `REQ-MM-013`: visibility variants carry the fixed UML notation prefixes.

use smith_core::Visibility;

#[test]
fn visibility_variants_carry_fixed_notation_prefixes() {
    use Visibility::{Package, Private, Protected, Public};
    assert_eq!(Public.notation(), "+");
    assert_eq!(Private.notation(), "-");
    assert_eq!(Protected.notation(), "#");
    assert_eq!(Package.notation(), "~");

    // The default visibility is public (metamodel core §NamedElement).
    assert_eq!(Visibility::default(), Visibility::Public);
}
