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

pub use commands::group;
pub(crate) use migration::migrate;
