//! Wire-side types for the `player_info` command group.

use arma_rs::IntoArma;
use serde::Serialize;

use crate::domain::PlayerId;

/// JSON row shape for [`Roster`]. `PlayerId`'s `Serialize` impl emits the
/// steam id as a JSON string (Steam IDs exceed JS safe-int range so
/// stringification is mandatory anyway). SQF `fromJSON` produces a `HashMap`
/// per row with `"steam_id"` / `"name"` keys.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct RosterEntry {
    pub steam_id: PlayerId,
    pub name: String,
}

/// Player roster — the canonical type for `player_info:list` data both
/// internally (return of `list_inner`) and on the wire. `IntoArma` emits the
/// whole list as a single JSON string so SQF consumes via `fromJSON _payload`
/// and gets a `Vec<HashMap>` directly. Serialization lives on the type; no
/// caller assembles the JSON manually.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct Roster(pub Vec<RosterEntry>);

impl IntoArma for Roster {
    fn to_arma(&self) -> arma_rs::Value {
        // serde_json::to_string on a Vec of simple structs (no maps with
        // non-string keys, no float NaN/Inf) is effectively infallible.
        arma_rs::Value::String(
            serde_json::to_string(&self.0).expect("Roster is infallible to serialize"),
        )
    }
}
