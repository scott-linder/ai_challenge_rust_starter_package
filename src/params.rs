use std::default::Default;
use error::{Result, Error};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Params {
    loadtime: i32,
    turntime: i32,
    rows: i32,
    cols: i32,
    turns: i32,
    viewradius2: i32,
    attackradius2: i32,
    spawnradius2: i32,
    player_seed: i64,
}

impl Params {
    pub fn new() -> Params {
        Default::default()
    }

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
