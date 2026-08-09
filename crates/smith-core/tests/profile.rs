//! `REQ-MM-010`: the built-in profiles are present and undeletable.

use smith_core::profile::{REQUIREMENTS_PROFILE, TESTS_PROFILE, TRACEABILITY_PROFILE};
use smith_core::{builtin_profiles, MetaclassKind};

#[test]
fn builtin_profiles_present_and_undeletable() {
    let profiles = builtin_profiles();
    assert_eq!(profiles.len(), 3);

    // Fixed order: traceability, requirements, tests.
    let names: Vec<&str> = profiles.iter().map(|p| p.qualified_name.as_str()).collect();
    assert_eq!(
        names,
        vec![TRACEABILITY_PROFILE, REQUIREMENTS_PROFILE, TESTS_PROFILE]
    );

    // Every built-in profile is permanent (present in every project, undeletable).
    for profile in &profiles {
        assert!(
            profile.is_builtin,
            "{} not marked builtin",
            profile.qualified_name
        );
    }

    // The traceability profile ships its seven trace stereotypes, each extending
    // Dependency (docs/architecture/04-traceability-relations.md).
    let traceability = &profiles[0];
    assert_eq!(traceability.stereotypes.len(), 7);
    // Exact stable identifiers from the normative taxonomy
    // (docs/architecture/04-traceability-relations.md). A typo here (e.g.
    // "deriveReq") would silently break downstream name-keyed trace matching.
    let trace_names: Vec<&str> = traceability
        .stereotypes
        .iter()
        .map(|s| s.name.as_str())
        .collect();
    assert_eq!(
        trace_names,
        vec![
            "satisfy",
            "verify",
            "realize",
            "deriveReqt",
            "refine",
            "trace",
            "copy"
        ]
    );
    for stereotype in &traceability.stereotypes {
        assert_eq!(
            stereotype.extended_metaclasses,
            vec![MetaclassKind::Dependency]
        );
    }
}
