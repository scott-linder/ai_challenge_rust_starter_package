use ant::Ant;
use player::Player;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Water,
    Food,
    Ant(Ant),
    Hill(Player),
    Land,
}

impl Tile {
    pub fn is_passable(&self) -> bool {
        match *self {
            Tile::Water => false,
            _ => true,
        }
    }

    pub fn is_unoccupied(&self) -> bool {
        match *self {
            Tile::Land | Tile::Ant(Ant { alive: false, .. }) => true,
            _ => false,
        }
    }
}
