extern crate ants;
extern crate rand;

use ants::ant::Ant;
use ants::bot::Bot;
use ants::error::Result;
use ants::player::Player;
use ants::tile::Tile;
use ants::world::World;
use rand::Rng;
use std::default::Default;

#[derive(Default)]
struct RandBot;

impl RandBot {
    pub fn new() -> RandBot {
        Default::default()
    }
}

impl Bot for RandBot {
    fn do_turn(&mut self, world: &mut World) -> Result<()> {
        let mut rng = rand::thread_rng();
        for (point, _) in world.map.tiles().filter(|&(_point, &tile)| {
                    tile == Some(Tile::Ant(Ant {
                        alive: true,
                        owner: Player::Me
                    }))
                }) {
            let direction = rng.gen();
            if world.map[point + direction].unwrap_or(Tile::Land).is_passable() {
                world.order(point, direction);
            }
        }
        Ok(())
    }
}

fn main() {
    let mut bot = RandBot::new();
    bot.run().unwrap();
}
