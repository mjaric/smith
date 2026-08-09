//! Visibility and its fixed UML notation prefix (`REQ-MM-013`).

/// UML visibility of a named element: `public`, `private`, `protected`,
/// or `package`.
///
/// The notation prefix is fixed by UML; Smith does not invent new visibilities.
/// The default, per the metamodel, is [`Visibility::Public`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Visibility {
    /// `+` — visible everywhere.
    #[default]
    Public,
    /// `-` — visible only within the owning namespace.
    Private,
    /// `#` — visible to specializations of the owner.
    Protected,
    /// `~` — visible within the nearest enclosing package.
    Package,
}

impl Visibility {
    /// The fixed UML notation prefix for this visibility.
    #[must_use]
    pub const fn notation(self) -> &'static str {
        match self {
            Self::Public => "+",
            Self::Private => "-",
            Self::Protected => "#",
            Self::Package => "~",
        }
    }
}
