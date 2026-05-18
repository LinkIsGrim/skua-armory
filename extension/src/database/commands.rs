//! Arma-callable database commands.

use arma_rs::Group;

use super::player::{player_connect, player_disconnect};
use super::pool::get_database_state;
use super::schema::bootstrap;

pub fn group() -> Group {
    Group::new()
        .command("bootstrap", bootstrap)
        .command("state", get_database_state)
        .command("player_connect", player_connect)
        .command("player_disconnect", player_disconnect)
}
