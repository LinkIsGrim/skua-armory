//! Player certifications.
//!
//! Backed by `skua_master.certifications` + `skua_master.player_certs`.
//! Certification definitions are reconciled from
//! `database/migrations/certifications/*.json` on every bootstrap; in-game
//! additions inserted after the last sync are preserved across reconciliations.

mod commands;
mod migration;
mod types;

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use types::Certification; // For watchdog tests

pub use commands::group;
pub(crate) use commands::{
    dispatch_grant_event, dispatch_revoke_event, get_player_inner, list_inner, push_list,
    push_player_certs,
};
pub(crate) use migration::migrate;
