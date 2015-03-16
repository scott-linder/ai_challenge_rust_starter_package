extern crate ants;

use ants::ant::Ant;
use ants::bot::Bot;
use ants::direction::Direction;
use ants::error::Result;
use ants::player::Player;
use ants::tile::Tile;
use ants::world::World;
use std::default::Default;

const DIRECTIONS: [Direction; 4] = [Direction::North, Direction::East,
                                    Direction::South, Direction::West];

#[derive(Default)]
struct LameBot;

impl LameBot {
    pub fn new() -> LameBot {
        Default::default()
    }
}

impl Bot for LameBot {
    fn do_turn(&mut self, world: &mut World) -> Result<()> {
        for direction in &DIRECTIONS {
            for (point, _) in world.map.tiles().filter(|&(_point, &tile)| {
                        tile == Some(Tile::Ant(Ant {
                            alive: true,
                            owner: Player::Me
                        }))
                    }) {
                if world.map[point + *direction].unwrap_or(Tile::Land).is_passable() {
                    world.order(point, *direction);
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut bot = LameBot::new();
    bot.run().unwrap();
}
