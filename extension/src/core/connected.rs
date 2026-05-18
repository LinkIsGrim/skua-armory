//! Set of currently-connected players, maintained by `database:player_connect`
//! and `database:player_disconnect`.
//!
//! Mirrors the SQF-side player presence so live-load and other push-on-change
//! flows can iterate "who needs to know" without round-tripping back to SQF.

use std::collections::HashSet;
use std::sync::{LazyLock, RwLock};

use crate::domain::PlayerId;

pub static CONNECTED: LazyLock<RwLock<HashSet<PlayerId>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));
