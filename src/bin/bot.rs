#![feature(core, io)]

extern crate ants;

use std::io::{stdin, BufRead, BufReadExt};
use ants::params::Params;
use ants::error::{Result, Error};
use ants::world::World;

fn parse_params<R: BufRead>(read: R) -> Result<Params> {
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

fn main() {
    let stdin = stdin();
    let params = parse_params(stdin.lock()).unwrap();
    let mut world = World::new(&params);
    println!("go");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "go" {
            println!("go");
            world.clear();
        } else if line.len() > 0 {
            world.update(&*line).unwrap();
        }
    }
}
