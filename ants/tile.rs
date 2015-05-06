//! Map tiles.

use ants::ant::Ant;
use ants::player::Player;

/// One tile on the game map.
///
/// A tile has a given position indexed by a `Point` into a `Map`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    /// Is (and always will be) impassable water.
    Water,
    /// Currently contains food.
    Food,
    /// Currently contains an ant (or many ants, if all dead).
    Ant(Ant),
    /// Currently contains an ant hill owned by the given `Player`.
    Hill(Player),
    /// Currently just vacant land.
    Land,
}

impl Tile {
    /// Whether the tile can be crossed by an ant (or could be at some point).
    pub fn is_passable(&self) -> bool {
        match *self {
            Tile::Water => false,
            _ => true,
        }
    }

    /// Whether the tile has nothing on it (except maybe dead ants).
    pub fn is_unoccupied(&self) -> bool {
        match *self {
            Tile::Land | Tile::Ant(Ant { alive: false, .. }) => true,
            _ => false,
        }
    }
}
