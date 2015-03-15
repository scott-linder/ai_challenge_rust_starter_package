//! Constant parameters set once per game.

use std::default::Default;
use error::{Result, Error};
use std::io::{BufRead, BufReadExt};

/// Parameters supplied once before the game begins.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct Params {
    pub loadtime: i32,
    pub turntime: i32,
    pub rows: i32,
    pub cols: i32,
    pub turns: i32,
    pub viewradius2: i32,
    pub attackradius2: i32,
    pub spawnradius2: i32,
    pub player_seed: i64,
}

impl Params {
    /// Construct a new `Params` with default values.
    pub fn new() -> Params {
        Default::default()
    }

    /// Parse input on `read` to populate a `Params`.
    ///
    /// Input must start with the line "turn 0", end with the line "ready",
    /// and have only valid parameter commands in-between.
    pub fn from_buf_read<R: BufRead>(read: R) -> Result<Params> {
        let mut lines = read.lines();
        let first_line = try!(try!(lines.next().ok_or(Error::UnexpectedEof)));
        if &*first_line != "turn 0" {
            return Err(Error::UnexpectedLine);
        }
        let mut params = Params::new();
        for line in lines {
            let line = try!(line);
            if line == "ready" {
                break;
            } else {
                try!(params.update(&*line));
            }
        }
        Ok(params)
    }

    /// Parse the given line for valid parameter commands and update self.
    pub fn update(&mut self, line: &str) -> Result<()> {
        match &*line.splitn(1, ' ').collect::<Vec<_>>() {
            [var, val] => match var {
                "loadtime" => self.loadtime = try!(val.parse()),
                "turntime" => self.turntime = try!(val.parse()),
                "rows" => self.rows = try!(val.parse()),
                "cols" => self.cols = try!(val.parse()),
                "turns" => self.turns = try!(val.parse()),
                "viewradius2" => self.viewradius2 = try!(val.parse()),
                "attackradius2" => self.attackradius2 = try!(val.parse()),
                "spawnradius2" => self.spawnradius2 = try!(val.parse()),
                "player_seed" => self.player_seed = try!(val.parse()),
                _ => return Err(Error::BadParameter),
            },
            _ => return Err(Error::BadParameter),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn new() {
        assert_eq!(Params::new(), Params {
            loadtime: 0i32,
            turntime: 0i32,
            rows: 0i32,
            cols: 0i32,
            turns: 0i32,
            viewradius2: 0i32,
            attackradius2: 0i32,
            spawnradius2: 0i32,
            player_seed: 0i64,
        });
    }

    #[test]
    fn example() {
        let mut params = Params::new();
        params.update("loadtime 3000").unwrap();
        params.update("turntime 1000").unwrap();
        params.update("rows 20").unwrap();
        params.update("cols 20").unwrap();
        params.update("turns 500").unwrap();
        params.update("viewradius2 55").unwrap();
        params.update("attackradius2 5").unwrap();
        params.update("spawnradius2 1").unwrap();
        params.update("player_seed 42").unwrap();
        assert_eq!(params, Params {
            loadtime: 3000,
            turntime: 1000,
            rows: 20,
            cols: 20,
            turns: 500,
            viewradius2: 55,
            attackradius2: 5,
            spawnradius2: 1,
            player_seed: 42,
            ..Default::default()
        });
    }
}
