//! Database connectivity and schema management.
//!
//! - Connection pool management ([`pool`])
//! - Schema bootstrap operations ([`schema`])
//! - Arma-callable commands ([`commands`])

mod commands;
mod player;
mod pool;
mod schema;
mod sql;
mod state;

#[cfg(test)]
mod tests;

pub use commands::group;
pub use player::PlayerInfo;
#[cfg(test)]
pub(crate) use pool::start_test_db;
pub use pool::{Database, DbSettings, get_client, get_db, get_state};
#[cfg(test)]
pub(crate) use schema::bootstrap_schema;
pub use state::DatabaseState;
