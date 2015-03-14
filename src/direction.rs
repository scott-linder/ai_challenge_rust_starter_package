use std::fmt;
use point::Point;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn into_point(self) -> Point {
        match self {
            Direction::North => Point { row: -1, col: 0 },
            Direction::South => Point { row: 1, col: 0 },
            Direction::East => Point { row: 0, col: 1 },
            Direction::West => Point { row: 0, col: -1 },
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", match *self {
            Direction::North => "n",
            Direction::South => "s",
            Direction::East => "e",
            Direction::West => "w",
        })
    }
}
