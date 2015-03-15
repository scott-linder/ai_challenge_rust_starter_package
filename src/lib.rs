#![feature(core, io, collections, plugin, custom_derive)]
#![plugin(rand_macros)]

extern crate rand;

pub mod error;
pub mod params;
pub mod world;
pub mod tile;
pub mod point;
pub mod player;
pub mod ant;
pub mod direction;
pub mod map;
pub mod bot;
