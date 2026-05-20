//! Read-only access to `skua_master.player_info` for SQF UIs.
//!
//! Writes are owned by `database::player` (`player_connect` upserts `name` and
//! bumps `last_seen`). This module just exposes a list endpoint admin tools
//! need to enumerate everyone who's ever played, online or not.

mod commands;
mod types;

#[cfg(test)]
mod tests;

pub use commands::group;
