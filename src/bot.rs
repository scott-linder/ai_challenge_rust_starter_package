//! Home of `Bot` trait, where user specifies their bot's unique behavior.

use world::World;
use error::Result;
use std::io::{stdin, BufReadExt};
use params::Params;

/// The `Bot` describes the unique functionality of all bots.
///
/// Every user of the crate should implement this trait somewhere.
///
/// The glue code to make the bot usable in the usual case is provided in a
/// default implementation of `run`, but it might be useful to provide a custom
/// implementation here as well.
pub trait Bot {
    /// The per-turn logic of the bot.
    ///
    /// This is called only once per game turn. By the time it is called, all
    /// input for that turn is processed.
    ///
    /// If the return is `Ok(())`, the world the turn is ended and the next
    /// turn begins; If it is `Err(..)`, the bot will exit.
    fn do_turn(&mut self, world: &mut World) -> Result<()>;

    /// The glue logic which maintains the world and calls `do_turn`.
    ///
    /// The default implementation should be sufficient for most purposes.
    fn run(&mut self) -> Result<()> {
        let stdin = stdin();
        let params = try!(Params::from_buf_read(stdin.lock()));
        let mut world = World::new(&params);
        println!("go");
        for line in stdin.lock().lines() {
            let line = try!(line);
            if line == "go" {
                try!(self.do_turn(&mut world));
                println!("go");
                world.clear();
            } else if line.len() > 0 {
                try!(world.update(&*line));
            }
        }
        Ok(())
    }
}
