use player::Player;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ant {
    pub alive: bool,
    pub owner: Player,
}
