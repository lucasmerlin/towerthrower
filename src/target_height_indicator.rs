use bevy::prelude::*;

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
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10000.0, 1.0)),
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, height, 0.0),
                ..Default::default()
            },
        ));
    }
}
