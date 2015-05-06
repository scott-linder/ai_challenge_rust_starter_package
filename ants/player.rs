//! Game players.

use std::default::Default;
use std::str::FromStr;
use ants::error::{Result, Error};

/// One game player.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    /// Special value representing our own bot.
    Me,
    /// All other players are represented by a small number.
    Other(u8),
}

impl Default for Player {
    fn default() -> Player {
        Player::Me
    }
}

impl FromStr for Player {
    type Err = Error;

    fn from_str(s: &str) -> Result<Player> {
        let i: u8 = try!(s.parse());
        Ok(match i {
            0 => Player::Me,
            n => Player::Other(n),
        })
    }
}
