mod components;
mod plugins;

pub use plugins::*;

pub mod prelude {
    pub use super::GameState;
    pub use crate::components::*;
    pub use crate::plugins::*;
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum GameState {
    Load,
    Main,
}

impl Default for GameState {
    fn default() -> Self {
        Self::Load
    }
}
