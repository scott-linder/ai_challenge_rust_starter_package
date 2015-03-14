use std::ops::{Index, IndexMut};
use std::slice;
use point::Point;
use tile::Tile;

#[derive(Debug)]
pub struct Map {
    rows: i32,
    cols: i32,
    tiles: Vec<Option<Tile>>,
}

impl Map {
    /// Create a new map, with fixed dimensions.
    pub fn new(rows: i32, cols: i32) -> Map {
        Map {
            rows: rows,
            cols: cols,
            tiles: vec![None; (rows * cols) as usize],
        }
    }

    pub fn tiles<'a>(&'a mut self) -> Tiles<'a> {
        Tiles { inner: self.tiles.iter_mut() }
    }
}

impl Index<Point> for Map {
    type Output = Option<Tile>;

    fn index<'a>(&'a self, point: &Point) -> &'a Option<Tile> {
        let Point{row, col} = point.wrap(self.rows, self.cols);
        &self.tiles[(row * self.cols + col) as usize]
    }
}

impl IndexMut<Point> for Map {
    fn index_mut<'a>(&'a mut self, point: &Point) -> &'a mut Option<Tile> {
        let Point{row, col} = point.wrap(self.rows, self.cols);
        &mut self.tiles[(row * self.cols + col) as usize]
    }
}

pub struct Tiles<'a> {
    inner: slice::IterMut<'a, Option<Tile>>,
}

impl<'a> Iterator for Tiles<'a> {
    type Item = &'a mut Option<Tile>;

    fn next(&mut self) -> Option<&'a mut Option<Tile>> {
        self.inner.next()
    }
}
