//! The entire game world.

use std::num::Float;
use tile::Tile;
use point::Point;
use ant::Ant;
use params::Params;
use error::{Result, Error};
use map::Map;
use direction::Direction;
use player::Player;

/// The entire game world.
///
/// All game state is tracked in something reachable via the `World`, and it
/// is the only thing passed to the bot during each turn.
#[derive(Debug)]
pub struct World<'a> {
    params: &'a Params,
    turn: i32,
    pub map: Map,
    vision_offsets: Box<[Point]>,
}

/// Calculate a set of "vision offsets": points relative to an ant that it
/// can see. This is an expensive operation, and so should be performed once
/// and cached (as the results are valid for the entirety of a game).
fn vision_offsets(params: &Params) -> Box<[Point]> {
    let mut offsets = Vec::new();
    let mx = (params.viewradius2 as f64).sqrt() as i32;
    for d_row in -mx..mx+1 {
        for d_col in -mx..mx+1 {
            let d = d_row * d_row + d_col * d_col;
            if 0 < d && d <= params.viewradius2 {
                offsets.push(Point {
                    row: d_row,
                    col: d_col,
                });
            }
        }
    }
    offsets.into_boxed_slice()
}

impl<'a> World<'a> {
    /// Construct a new `World` from the given game parameters.
    pub fn new(params: &Params) -> World {
        World {
            params: params,
            turn: 0,
            map: Map::new(params.rows, params.cols),
            vision_offsets: vision_offsets(params),
        }
    }

    /// Clear all tiles, except water (which never changes between turns).
    pub fn clear(&mut self) {
        for (_, tile) in self.map.tiles_mut() {
            if *tile != Some(Tile::Water) {
                *tile = None;
            }
        }
    }

    /// Update vision relative to the given `point` using `vision_offsets`.
    fn update_vision(&mut self, point: Point) {
        for offset in self.vision_offsets.iter() {
            let visible = point + *offset;
            // only update visibility of tiles which are not already visible
            if let None = self.map[visible] {
                self.map[visible] = Some(Tile::Land);
            }
        }
    }

    /// Update world from one line of input.
    ///
    /// Blank and "go" lines are not handled here and should not be passed in.
    /// Vision is updated automatically as ants are discovered.
    pub fn update(&mut self, line: &str) -> Result<()> {
        let splits = line.split(' ').collect::<Vec<_>>();
        match &*splits {
            ["turn", turn] => {
                self.turn = try!(turn.parse());
            },
            [variant, row, col, rest..] => {
                let point = Point {
                    row: try!(row.parse()),
                    col: try!(col.parse()),
                };
                let tile = match variant {
                    "w" => Some(Tile::Water),
                    "f" => Some(Tile::Food),
                    _ => match rest {
                        [owner] => {
                            let owner = try!(owner.parse());
                            match variant {
                                "h" => Some(Tile::Hill(owner)),
                                "a" => Some(Tile::Ant(Ant {
                                    alive: true,
                                    owner: owner
                                })),
                                "d" => Some(Tile::Ant(Ant {
                                    alive: false,
                                    owner: owner
                                })),
                                _ => return Err(Error::UnknownCommand),
                            }
                        },
                        _ => return Err(Error::UnknownCommand),
                    },
                };
                if let Some(Tile::Ant(Ant { owner: Player::Me, .. })) = tile {
                    self.update_vision(point);
                }
                self.map[point] = tile;
            },
            _ => return Err(Error::UnexpectedLine),
        }
        Ok(())
    }

    /// Check what turn it is.
    pub fn turn(&self) -> i32 {
        self.turn
    }

    /// Issue order for an ant at a given point to move in the given direction.
    pub fn order(&self, point: Point, direction: Direction) {
        println!("o {} {}", point, direction);
    }
}
