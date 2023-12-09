use crate::state::LevelState;
use crate::VERTICAL_VIEWPORT_SIZE;
use bevy::prelude::*;

pub struct CityPlugin;

impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), (setup_city));
    }
}

pub fn setup_city(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(0.0, 20.0, 100.0),
            texture: assets.load("foreground.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    VERTICAL_VIEWPORT_SIZE * 16.0 / 9.0,
                    VERTICAL_VIEWPORT_SIZE,
                )),
                ..Default::default()
            },
            ..Default::default()
        }),
    );

    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -100.0),
            texture: assets.load("background.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    VERTICAL_VIEWPORT_SIZE * 16.0 / 9.0,
                    VERTICAL_VIEWPORT_SIZE,
                )),
                ..Default::default()
            },
            ..Default::default()
        }),
    );
}
