//! Element identity: opaque, immutable, UUIDv7-backed (`REQ-MM-001`).

use uuid::Uuid;

/// Opaque, globally-unique, immutable identity for every model element.
///
/// Backed by a `UUIDv7` (48-bit millisecond timestamp + random tail), minted at
/// creation. The inner value is private: there is no accessor that can mutate
/// it, so identity is immutable for an element's lifetime. Uniqueness holds by
/// construction of `UUIDv7` generation (`INV-MM-002`): a collision requires the
/// same timestamp *and* 74 random bits to coincide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementId(Uuid);

impl ElementId {
    /// Mint a fresh, time-ordered identity (`Uuid::now_v7`).
    ///
    /// The `UUIDv7` layout places the Unix-millisecond timestamp in the leading
    /// 48 bits, so identities minted in sequence have a non-decreasing
    /// timestamp prefix.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// The 16 raw `UUIDv7` bytes (timestamp in the leading 6 bytes).
    #[must_use]
    pub const fn as_bytes(&self) -> [u8; 16] {
        *self.0.as_bytes()
    }

    /// The underlying [`Uuid`].
    #[must_use]
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for ElementId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for ElementId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<ElementId> for Uuid {
    fn from(value: ElementId) -> Self {
        value.0
    }
}
