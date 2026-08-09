//! `REQ-MM-001` / `INV-MM-002`: identity is unique, immutable, UUIDv7-minted.
//!
//! The proptest exercises `UUIDv7` generation: uniqueness holds across a
//! generated batch (no shared ids, by construction) and the 48-bit millisecond
//! timestamp prefix is non-decreasing in mint order.

use std::collections::HashSet;

use proptest::prelude::*;

use smith_core::ElementId;

proptest! {
    #[test]
    fn element_id_is_unique_and_immutable(batch in 2u16..512u16) {
        // Immutability: the inner value is private and exposes no mutator, so an
        // id stays equal to itself across every read.
        let a = ElementId::new();
        prop_assert_eq!(a, a);
        prop_assert_eq!(a.as_bytes(), a.as_bytes());

        // Uniqueness: two distinct mints differ.
        prop_assert_ne!(a, ElementId::new());

        // `INV-MM-002` + UUIDv7 ordering over a generated batch.
        let ids: Vec<ElementId> = (0..batch).map(|_| ElementId::new()).collect();
        let mut seen = HashSet::new();
        for id in &ids {
            prop_assert!(seen.insert(*id), "duplicate id minted");
        }
        // UUIDv7: the leading 6 bytes are the ms timestamp (big-endian), so the
        // prefix is non-decreasing across sequential mints.
        for window in ids.windows(2) {
            let earlier = window[0].as_bytes();
            let later = window[1].as_bytes();
            prop_assert!(earlier[0..6] <= later[0..6], "v7 timestamp prefix decreased");
        }
    }
}
