use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::block::{Aiming, Block, Falling};
use crate::throw::TargetIndicator;

pub struct MagneticPlugin;

impl Plugin for MagneticPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, magnetic_effect_system);
    }
}

#[derive(Component, Debug)]
pub struct MagneticEffect {
    pub range: f32,
    pub force: f32,
}

impl Default for MagneticEffect {
    fn default() -> Self {
        Self {
            range: 200.0,
            force: 2.0,
        }
    }
}

/// Should apply a force to all blocks in range, pulling them towards the center of the block
pub fn magnetic_effect_system(
    mut magnets_query: Query<
        (&Transform, &mut ExternalImpulse, &MagneticEffect),
        (Without<Falling>, Without<Aiming>, Without<TargetIndicator>),
    >,
    mut blocks_query: Query<
        (&Transform, &mut ExternalImpulse),
        (Without<MagneticEffect>, With<Block>),
    >,
) {
    for ((magnet_transform, mut magnet_self_impulse, effect)) in magnets_query.iter_mut() {
        for (block_transform, mut external_impulse) in blocks_query.iter_mut() {
            let distance = magnet_transform
                .translation
                .distance(block_transform.translation);
            if distance < effect.range {
                let direction =
                    (magnet_transform.translation - block_transform.translation).normalize();

                // max force at 0 distance, 0 force at max distance
                let force = (effect.range - distance) / effect.range * effect.force;

                let force = direction * force;
                external_impulse.impulse = force.xy();
                magnet_self_impulse.impulse -= force.xy();
            }
        }
    }
}
