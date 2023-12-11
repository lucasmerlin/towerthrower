use crate::ASSET_SCALE;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::level::{Level, LevelGoal, LevelLifecycle};
use crate::state::LevelState;

pub struct TargetHeightIndicatorPlugin;

impl Plugin for TargetHeightIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelState::Playing),
            (setup_target_height_indicator,),
        );
    }
}

#[derive(Component, Debug)]
pub struct TargetHeightIndicator;

pub fn setup_target_height_indicator(
    mut commands: Commands,
    mut level: Res<Level>,
    old_indicator: Query<Entity, With<TargetHeightIndicator>>,
    assets: Res<AssetServer>,
) {
    println!("Setting up target height indicator");
    for entity in old_indicator.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if let LevelGoal::ReachHeight(height) = level.goal {
        commands.spawn((
            TargetHeightIndicator,
            LevelLifecycle,
            SpriteBundle {
                transform: Transform::from_xyz(0.0, height, -0.1).with_scale(Vec3::new(
                    ASSET_SCALE,
                    ASSET_SCALE * 0.5,
                    1.0,
                )),
                texture: assets.load("target_height_indicator.png"),
                sprite: Sprite {
                    anchor: Anchor::TopCenter,
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
}
