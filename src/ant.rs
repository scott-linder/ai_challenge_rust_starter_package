use player::Player;

#[derive(Default, Debug, Copy, PartialEq, Eq)]
pub struct Ant {
    alive: bool,
    owner: Player,
}
