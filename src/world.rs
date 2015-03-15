use std::num::Float;
use tile::Tile;
use point::Point;
use ant::Ant;
use params::Params;
use error::{Result, Error};
use map::Map;
use direction::Direction;

#[derive(Debug)]
pub struct World<'a> {
    params: &'a Params,
    turn: i32,
    pub map: Map,
    vision_offsets: Box<[Point]>,
}

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
    pub fn new(params: &Params) -> World {
        World {
            params: params,
            turn: 0,
            map: Map::new(params.rows, params.cols),
            vision_offsets: vision_offsets(params),
        }
    }

    pub fn clear(&mut self) {
        for (_, tile) in self.map.tiles_mut() {
            if *tile != Some(Tile::Water) {
                *tile = None;
            }
        }
    }

    fn update_vision(&mut self, point: Point) {
        for offset in self.vision_offsets.iter() {
            let visible = point + *offset;
            // only update visibility of tiles which are not already visible
            if let None = self.map[visible] {
                self.map[visible] = Some(Tile::Land);
            }
        }
    }

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
                self.map[point] = match variant {
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
                if let Some(Tile::Ant(..)) = self.map[point] {
                    self.update_vision(point);
                }
            },
            _ => return Err(Error::UnexpectedLine),
        }
        Ok(())
    }

    pub fn turn(&self) -> i32 {
        self.turn
    }

    pub fn order(&self, point: Point, direction: Direction) {
        println!("o {} {}", point, direction);
    }
}
