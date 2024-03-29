use bevy::prelude::*;
use strum::EnumIter;

pub(super) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}

// All the app's possible states.
#[derive(
    Clone, Copy, Debug, Default, EnumIter, Eq, Hash, PartialEq, States,
)]
pub enum GameState {
    #[default]
    AssetLoading,
    StartMenu,
    Paused,
    Gameplay,
    GameOver,
}
