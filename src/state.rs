use crate::level::LevelLifecycle;
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LevelState>()
            .add_state::<GameState>()
            .add_systems(OnEnter(LevelState::Loading), enter_playing_state)
            .add_systems(OnEnter(LevelState::Won), play_win_sound_system);
    }
}

#[derive(States, Debug, Default, Hash, Clone, Eq, PartialEq)]
pub enum LevelState {
    #[default]
    Loading,
    Playing,
    Lost,
    Won,
    KeepPlaying,
}

#[derive(States, Debug, Default, Hash, Clone, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
}

fn enter_playing_state(
    mut commands: Commands,
    state: Res<State<LevelState>>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut all_query: Query<(Entity), With<LevelLifecycle>>,
) {
    if *state == LevelState::Loading {
        for entity in all_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(LevelState::Playing);
    }
}

fn play_win_sound_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((AudioBundle {
        source: asset_server.load("sounds/victory.wav"),
        settings: PlaybackSettings::DESPAWN,
    },));
}
