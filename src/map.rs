//! 2D representation of the game world's map.

use std::ops::{Index, IndexMut};
use std::slice;
use std::iter::Enumerate;
use point::Point;
use tile::Tile;

/// 2D collection of `Tile` with fixed dimensions and visibility.
///
/// Visibility is represented by wrapping each `Tile` in an `Option`: `None`
/// represents a tile which is not visible, and `Some(..)` wraps one which is.
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

    /// An iterator over all tiles in the map, in no particular order.
    pub fn tiles<'a>(&'a self) -> Tiles<'a> {
        Tiles {
            cols: self.cols,
            inner: self.tiles.iter().enumerate(),
        }
    }

    /// A mut iterator over all tiles in the map, in no particular order.
    pub fn tiles_mut<'a>(&'a mut self) -> TilesMut<'a> {
        TilesMut {
            cols: self.cols,
            inner: self.tiles.iter_mut().enumerate(),
        }
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

/// An iterator over all tiles in the map, in no particular order.
pub struct Tiles<'a> {
    cols: i32,
    inner: Enumerate<slice::Iter<'a, Option<Tile>>>,
}

impl<'a> Iterator for Tiles<'a> {
    type Item = (Point, &'a Option<Tile>);

    fn next(&mut self) -> Option<(Point, &'a Option<Tile>)> {
        if let Some((i, tile)) = self.inner.next() {
            Some((Point {
                row: i as i32 / self.cols,
                col: i as i32 % self.cols,
            }, tile))
        } else {
            None
        }
    }
}

/// A mut iterator over all tiles in the map, in no particular order.
pub struct TilesMut<'a> {
    cols: i32,
    inner: Enumerate<slice::IterMut<'a, Option<Tile>>>,
}

impl<'a> Iterator for TilesMut<'a> {
    type Item = (Point, &'a mut Option<Tile>);

    fn next(&mut self) -> Option<(Point, &'a mut Option<Tile>)> {
        if let Some((i, tile)) = self.inner.next() {
            Some((Point {
                row: i as i32 / self.cols,
                col: i as i32 % self.cols,
            }, tile))
        } else {
            None
        }
    }
}
