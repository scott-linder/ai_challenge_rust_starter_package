use world::World;
use error::Result;
use std::io::{stdin, BufReadExt};
use params::Params;

pub trait Bot {
    fn do_turn(&mut self, world: &mut World) -> Result<()>;

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
