use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LevelState>().add_state::<GameState>();
    }
}

#[derive(States, Debug, Default, Hash, Clone, Eq, PartialEq)]
pub enum LevelState {
    #[default]
    Playing,
    Dead,
    Won,
}

#[derive(States, Debug, Default, Hash, Clone, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}
