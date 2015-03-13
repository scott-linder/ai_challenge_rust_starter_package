use std::default::Default;
use std::collections::{HashMap, HashSet};
use tile::Tile;
use player::Player;
use ant::Ant;
use params::Params;
use error::{Result, Error};

#[derive(Debug)]
pub struct World<'a> {
    params: &'a Params,
    pub water: HashSet<Tile>,
    pub food: HashSet<Tile>,
    pub ants: HashMap<Tile, Ant>,
    pub hills: HashMap<Tile, Player>,
}

impl<'a> World<'a> {
    pub fn new(params: &Params) -> World {
        World {
            params: params,
            water: Default::default(),
            food: Default::default(),
            ants: Default::default(),
            hills: Default::default(),
        }
    }

    /// Clear all fields except water.
    pub fn clear(&mut self) -> World {
        self.food.clear();
        self.ants.clear();
        self.hills.clear();
    }

    pub fn update(&mut self, line: &str) -> Result<()> {
        let splits = line.split(' ').collect::<Vec<_>>();
        match &*splits {
            [variant, row, col, rest..] => {
                let tile = Tile {
                    row: try!(row.parse()),
                    col: try!(col.parse()),
                };
                match variant {
                    "w" => { self.water.insert(tile); },
                    "f" => { self.food.insert(tile); },
                    _ => match rest {
                        [owner] => {
                            let owner = try!(owner.parse());
                            match variant {
                                "h" => { self.hills.insert(tile, owner); },
                                "a" => {
                                    self.ants.insert(tile, Ant {
                                        alive: true,
                                        owner: owner,
                                    });
                                },
                                "d" => {
                                    self.ants.insert(tile, Ant {
                                        alive: false,
                                        owner: owner,
                                    });
                                },
                                _ => return Err(Error::UnknownCommand),
                            }
                        },
                        _ => return Err(Error::UnknownCommand),
                    },
                }
            },
            _ => return Err(Error::UnexpectedLine),
        }
        Ok(())
    }
}
