//! Rank value types.

use arma_rs::IntoArma;
use serde::{Deserialize, Serialize};

/// Row shape returned by `ranks:list`. `id` matches `player_info.rank` (a
/// `SMALLINT` FK into `ranks`).
///
/// `Serialize` is used by the event module so `RankListChanged` can ship a
/// JSON-encoded list of ranks to SQF via `Event::payload`.
#[derive(Debug, Clone, PartialEq, Eq, IntoArma, Serialize)]
pub struct Rank {
    pub id: i16,
    pub display_name: String,
}

/// JSON file shape under `database/migrations/ranks/<NN_slug>.json`. The id
/// is authored inside the JSON; the filename is only a sort hint.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RankFile {
    pub id: i16,
    pub display_name: String,
}
