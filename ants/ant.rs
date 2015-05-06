//! Ants currently in play.
//!
//! Representation and any code related to individual ants.

use ants::player::Player;

/// An Ant that is currently in play.
///
/// As a note: a dead ant can be assumed to represent any number of dead ants,
/// as they can stack. It is assumed this level of detail is typically not
/// important, so we use a `bool` flag.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ant {
    /// Whether the ant is alive or dead.
    pub alive: bool,
    /// Which player controls this ant.
    pub owner: Player,
}
