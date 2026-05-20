//! Certification value types.

use arma_rs::IntoArma;
use serde::{Deserialize, Serialize};

use crate::domain::PlayerId;

/// Payload type for `certification:get_player`. Carries the grantee UID
/// alongside their cert ids so the SQF callback can route the response back
/// to whichever player it asked about (the wire envelope's `_function` slot
/// is just `"get_player"` — no per-request identity otherwise). `IntoArma`
/// emits the whole payload as one JSON string; SQF consumes via `fromJSON`.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct PlayerCerts {
    pub player_id: PlayerId,
    pub cert_ids: Vec<String>,
}

impl IntoArma for PlayerCerts {
    fn to_arma(&self) -> arma_rs::Value {
        arma_rs::Value::String(
            serde_json::to_string(self).expect("PlayerCerts is infallible to serialize"),
        )
    }
}

/// Row shape returned by `certification:list`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Certification {
    pub id: String,
    pub display_name: String,
    pub document: String,
    pub description: String,
    pub perk: String,
    pub pay_bonus: i32,
    pub grant_event: String,
    pub revoke_event: String,
    pub requires: Vec<String>,
}

/// JSON file shape under `database/migrations/certifications/<id>.json`.
/// The file's stem provides the `id`; the JSON body provides the rest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CertificationFile {
    pub display_name: String,
    pub document: String,
    pub description: String,
    pub perk: String,
    pub pay_bonus: i32,
    pub grant_event: String,
    pub revoke_event: String,
    pub requires: Vec<String>,
}
