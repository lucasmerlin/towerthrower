use crate::base::Base;
use crate::block::{CaughtBlock, Falling};
use crate::effect::magnetic::MagneticEffect;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlatformEffectPlugin;

impl Plugin for PlatformEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, platform_remove_physics_system);
    }
}

/// Causes the entity to move left and right in sync with the base platform
/// Helps stabilize the pile
#[derive(Component, Debug, Default)]
pub struct PlatformEffect;

pub fn platform_remove_physics_system(
    mut events: EventReader<CaughtBlock>,
    mut commands: Commands,
    mut query: Query<Entity, With<PlatformEffect>>,
) {
    for event in events.read() {
        if let Ok(entity) = query.get_mut(event.entity) {
            commands.entity(entity).insert(RigidBody::Fixed);
        }
    }
}

// pub fn platform_effect_system(
//     mut query: Query<(&mut Velocity), (With<PlatformEffect>, Without<Base>, Without<Falling>)>,
//     base_query: Query<&Velocity, With<Base>>,
// ) {
//     let base_velocity = base_query.single();
//     for (mut velocity) in query.iter_mut() {
//         *velocity = base_velocity.clone();
//     }
// }
