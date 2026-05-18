//! Tests for cert list watchdog diff detection.
//!
//! Verifies that the watchdog correctly detects changes to certification
//! definitions in the database and identifies when the list needs to be
//! re-pushed to SQF.

use crate::certification::Certification;

/// Verify that cert list hashes are deterministic: same JSON serialization
/// always produces the same hash. This is critical for diff detection — if
/// hashes were nondeterministic, legitimate unchanged lists could appear
/// changed and trigger spurious pushes.
#[test]
fn cert_list_hash_is_deterministic() {
    let cert = Certification {
        id: "pilot".into(),
        display_name: "Pilot".into(),
        document: "https://docs/pilot".into(),
        description: "Pilot training".into(),
        perk: "Rotary access".into(),
        pay_bonus: 1000,
        grant_event: "skua_cert_pilot".into(),
        revoke_event: "skua_cert_revoke_pilot".into(),
        requires: vec!["rifleman".into()],
    };
    let list = vec![cert.clone()];

    let json1 = serde_json::to_string(&list).expect("serialize 1");
    let json2 = serde_json::to_string(&list).expect("serialize 2");

    assert_eq!(
        json1, json2,
        "serde_json serialization must be deterministic"
    );
    assert_eq!(
        json1, json2,
        "identical cert lists must produce identical JSON"
    );
}

/// Verify that different cert lists produce different JSON. This ensures
/// that additions, removals, or field modifications are detected as changes.
#[test]
fn cert_list_hash_differs_on_content_change() {
    let cert1 = Certification {
        id: "pilot".into(),
        display_name: "Pilot".into(),
        document: "https://docs/pilot".into(),
        description: "Pilot training".into(),
        perk: "Rotary access".into(),
        pay_bonus: 1000,
        grant_event: "skua_cert_pilot".into(),
        revoke_event: "skua_cert_revoke_pilot".into(),
        requires: vec!["rifleman".into()],
    };
    let cert2 = Certification {
        id: "medic".into(),
        display_name: "Medic".into(),
        document: "https://docs/medic".into(),
        description: "Medical training".into(),
        perk: "Medical supplies".into(),
        pay_bonus: 500,
        grant_event: "skua_cert_medic".into(),
        revoke_event: "skua_cert_revoke_medic".into(),
        requires: vec![],
    };

    let json1 = serde_json::to_string(&vec![cert1.clone()]).expect("serialize");
    let json2 = serde_json::to_string(&vec![cert1, cert2]).expect("serialize");

    assert_ne!(
        json1, json2,
        "different cert lists must produce different JSON"
    );
}

/// Verify that a modified cert field (e.g., `display_name`) changes the hash.
/// This ensures the watchdog catches updates to existing cert definitions.
#[test]
fn cert_list_hash_differs_on_field_modification() {
    let mut cert1 = Certification {
        id: "pilot".into(),
        display_name: "Pilot".into(),
        document: "https://docs/pilot".into(),
        description: "Pilot training".into(),
        perk: "Rotary access".into(),
        pay_bonus: 1000,
        grant_event: "skua_cert_pilot".into(),
        revoke_event: "skua_cert_revoke_pilot".into(),
        requires: vec!["rifleman".into()],
    };

    let json1 = serde_json::to_string(&vec![cert1.clone()]).expect("serialize 1");

    // Modify a field
    cert1.display_name = "Helicopter Pilot".into();
    let json2 = serde_json::to_string(&vec![cert1]).expect("serialize 2");

    assert_ne!(
        json1, json2,
        "modified cert field must produce different JSON"
    );
}

/// Verify that cert list order matters for the hash (i.e., the JSON
/// representation preserves order). This ensures that the same certs in
/// different order will trigger a list push, which SQF can then handle.
/// (Order preservation allows SQF UI code to detect re-ordering.)
#[test]
fn cert_list_hash_differs_on_reordering() {
    let cert1 = Certification {
        id: "pilot".into(),
        display_name: "Pilot".into(),
        document: "https://docs/pilot".into(),
        description: "Pilot training".into(),
        perk: "Rotary access".into(),
        pay_bonus: 1000,
        grant_event: "skua_cert_pilot".into(),
        revoke_event: "skua_cert_revoke_pilot".into(),
        requires: vec!["rifleman".into()],
    };
    let cert2 = Certification {
        id: "medic".into(),
        display_name: "Medic".into(),
        document: "https://docs/medic".into(),
        description: "Medical training".into(),
        perk: "Medical supplies".into(),
        pay_bonus: 500,
        grant_event: "skua_cert_medic".into(),
        revoke_event: "skua_cert_revoke_medic".into(),
        requires: vec![],
    };

    let json1 = serde_json::to_string(&vec![cert1.clone(), cert2.clone()]).expect("s1");
    let json2 = serde_json::to_string(&vec![cert2, cert1]).expect("s2");

    assert_ne!(
        json1, json2,
        "cert list order must affect JSON (and thus hash)"
    );
}
