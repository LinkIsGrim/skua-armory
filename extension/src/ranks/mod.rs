//! Player ranks.
//!
//! Backed by `skua_master.ranks` + `skua_master.player_info.rank`.
//! Rank definitions are reconciled from `database/migrations/ranks/*.json` on
//! every bootstrap; in-game additions inserted after the last sync are
//! preserved across reconciliations.

mod commands;
mod migration;
mod types;

#[cfg(test)]
mod tests;

pub use commands::group;
pub(crate) use commands::push_list;
pub(crate) use migration::migrate;
pub(crate) use types::Rank;
