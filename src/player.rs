use std::default::Default;
use std::str::FromStr;
use error::Result;

#[derive(Debug, Copy, PartialEq, Eq)]
pub enum Player {
    Me,
    Other(u8),
}

impl Default for Player {
    fn default() -> Player {
        Player::Me
    }
}

impl FromStr for Player {
    fn from_str(s: &str) -> Result<Player> {
        let i: u8 = try!(s.parse());
        match i {
            0 => Player::Me,
            n => Player::Other(n),
        }
    }
}
