//! Unit tests for the event module.
//!
//! These tests freeze the wire shape SQF consumers depend on. If a field name
//! changes, every SQF handler that reads `_data get "..."` breaks silently in
//! production — these assertions are the cheap guardrail.

use chrono::{TimeZone, Utc};
use serde_json::Value;

use super::*;
use crate::certification::Certification;
use crate::database::PlayerInfo;
use crate::ranks::Rank;

fn parse(s: &str) -> Value {
    serde_json::from_str(s).expect("payload not valid JSON")
}

#[test]
fn event_names_are_stable() {
    // Locks the wire names. Changing any of these breaks every SQF subscriber
    // that uses the corresponding `QEV_*` macro — and the enum_sync test
    // catches the macro side, but only if these stay in sync with the table
    // there too.
    let cases = [
        (
            Event::CertificationGranted {
                player_id: PlayerId::from_i64(1),
                cert_id: "x".into(),
            },
            "skua_certification_granted",
        ),
        (
            Event::CertificationRevoked {
                player_id: PlayerId::from_i64(1),
                cert_id: "x".into(),
            },
            "skua_certification_revoked",
        ),
        (
            Event::CertificationListChanged { certs: vec![] },
            "skua_certification_list_changed",
        ),
        (
            Event::RankChanged {
                player_id: PlayerId::from_i64(1),
                rank_id: 0,
            },
            "skua_rank_changed",
        ),
        (
            Event::RankListChanged { ranks: vec![] },
            "skua_rank_list_changed",
        ),
        (
            Event::PlayerDisconnected {
                player_id: PlayerId::from_i64(1),
            },
            "skua_player_disconnected",
        ),
    ];
    for (event, expected) in cases {
        assert_eq!(event.event_name(), expected);
    }
}

#[test]
fn only_player_connected_is_critical() {
    let info = PlayerInfo {
        steam_id: PlayerId::from_i64(76_561_198_000_000_000),
        name: "x".into(),
        first_seen: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
        last_seen: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
        is_admin: false,
        is_banned: false,
        rank: 0,
    };
    assert!(Event::PlayerConnected { info }.is_critical());
    assert!(
        !Event::CertificationGranted {
            player_id: PlayerId::from_i64(1),
            cert_id: "x".into()
        }
        .is_critical()
    );
    assert!(
        !Event::PlayerDisconnected {
            player_id: PlayerId::from_i64(1)
        }
        .is_critical()
    );
}

#[test]
fn cert_event_payload_has_player_id_and_cert_id() {
    let event = Event::CertificationGranted {
        player_id: PlayerId::from_i64(42),
        cert_id: "medic".into(),
    };
    let payload = parse(&event.payload().unwrap());
    assert!(payload.get("player_id").is_some(), "missing player_id");
    assert_eq!(payload.get("cert_id"), Some(&Value::String("medic".into())));
}

#[test]
fn rank_changed_payload_has_player_id_and_rank_id() {
    let event = Event::RankChanged {
        player_id: PlayerId::from_i64(42),
        rank_id: 3,
    };
    let payload = parse(&event.payload().unwrap());
    assert!(payload.get("player_id").is_some());
    assert_eq!(payload.get("rank_id"), Some(&Value::Number(3.into())));
}

#[test]
fn player_disconnected_payload_has_player_id() {
    let event = Event::PlayerDisconnected {
        player_id: PlayerId::from_i64(42),
    };
    let payload = parse(&event.payload().unwrap());
    assert!(payload.get("player_id").is_some());
}

#[test]
fn list_changed_payloads_are_top_level_arrays() {
    let certs = Event::CertificationListChanged { certs: vec![] };
    let parsed = parse(&certs.payload().unwrap());
    assert!(
        matches!(parsed, Value::Array(_)),
        "CertificationListChanged payload must be a top-level JSON array, got {parsed:?}"
    );

    let ranks = Event::RankListChanged { ranks: vec![] };
    let parsed = parse(&ranks.payload().unwrap());
    assert!(
        matches!(parsed, Value::Array(_)),
        "RankListChanged payload must be a top-level JSON array, got {parsed:?}"
    );
}

#[test]
fn rank_list_payload_preserves_id_and_display_name() {
    let event = Event::RankListChanged {
        ranks: vec![Rank {
            id: 1,
            display_name: "Captain".into(),
        }],
    };
    let parsed = parse(&event.payload().unwrap());
    let array = parsed.as_array().expect("array");
    let row = &array[0];
    assert_eq!(row.get("id"), Some(&Value::Number(1.into())));
    assert_eq!(
        row.get("display_name"),
        Some(&Value::String("Captain".into()))
    );
}

#[test]
fn cert_list_payload_preserves_existing_fields() {
    // Matches the shape that addons/certifications/fnc_onCertificationListChanged
    // reads via fromJSON.
    let event = Event::CertificationListChanged {
        certs: vec![Certification {
            id: "medic".into(),
            display_name: "Medic".into(),
            document: "doc".into(),
            description: "desc".into(),
            perk: "perk".into(),
            pay_bonus: 100,
            grant_event: "ge".into(),
            revoke_event: "re".into(),
            requires: vec!["other".into()],
        }],
    };
    let parsed = parse(&event.payload().unwrap());
    let row = &parsed.as_array().expect("array")[0];
    for field in [
        "id",
        "display_name",
        "document",
        "description",
        "perk",
        "pay_bonus",
        "grant_event",
        "revoke_event",
        "requires",
    ] {
        assert!(row.get(field).is_some(), "missing field {field}");
    }
}

#[test]
fn player_connected_payload_has_player_info_fields() {
    let info = PlayerInfo {
        steam_id: PlayerId::from_i64(76_561_198_000_000_000),
        name: "Alice".into(),
        first_seen: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
        last_seen: Utc.with_ymd_and_hms(2020, 6, 1, 12, 0, 0).unwrap(),
        is_admin: true,
        is_banned: false,
        rank: 2,
    };
    let event = Event::PlayerConnected { info };
    let parsed = parse(&event.payload().unwrap());
    for field in [
        "steam_id",
        "name",
        "first_seen",
        "last_seen",
        "is_admin",
        "is_banned",
        "rank",
    ] {
        assert!(parsed.get(field).is_some(), "missing field {field}");
    }
}

#[test]
fn internal_bus_delivers_events_to_subscribers() {
    let mut rx = subscribe();
    let event = Event::PlayerDisconnected {
        player_id: PlayerId::from_i64(1),
    };
    // Emit via the bus directly; no Context required for this test.
    let _ = BUS.send(event.clone());
    let received = rx.try_recv().expect("bus did not deliver event");
    match received {
        Event::PlayerDisconnected { player_id } => assert_eq!(player_id, PlayerId::from_i64(1)),
        other => panic!("unexpected variant: {other:?}"),
    }
}
