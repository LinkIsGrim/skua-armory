//! Per-player loadout persistence.
//!
//! Stores the player's CBA loadout in `<campaign_schema>.player_data.loadout`
//! and falls back to `skua_master.campaigns.default_loadout` when the player
//! has no row yet. All loadout strings round-trip through [`arma_rs::Loadout`]
//! so corrupt rows can't reach disk and stored rows are always
//! `CBA_fnc_setLoadout`-compatible on read.

mod commands;
mod storage;

#[cfg(test)]
mod tests;

pub use commands::group;
