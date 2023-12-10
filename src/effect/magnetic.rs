use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::block::{Aiming, Block, Falling};
use crate::throw::TargetIndicator;

pub struct MagneticPlugin;

impl Plugin for MagneticPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, magnetic_effect_system);
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
            range: 10.0,
            force: 2.0,
        }
    }
}

/// Should apply a force to all blocks in range, pulling them towards the center of the block
pub fn magnetic_effect_system(
    mut magnets_query: Query<
        (Entity, &Transform, &mut ExternalImpulse, &MagneticEffect),
        (Without<Aiming>, Without<TargetIndicator>, With<Block>),
    >,
    mut blocks_query: Query<
        (Entity, &Transform, &mut ExternalImpulse),
        (
            With<Block>,
            Without<MagneticEffect>,
            Without<Aiming>,
            Without<TargetIndicator>,
        ),
    >,
) {
    for ((mag_entity, magnet_transform, mut magnet_self_impulse, effect)) in
        magnets_query.iter_mut()
    {
        for (block_entity, block_transform, mut external_impulse) in blocks_query.iter_mut() {
            let impulse = calculate_magnetic_impulse(magnet_transform, block_transform, effect);

            if let Some(impulse) = impulse {
                external_impulse.impulse = impulse;
                magnet_self_impulse.impulse -= impulse;
            }
        }
    }

    let mut combinations = magnets_query.iter_combinations_mut();
    while let Some(
        [(entity_a, transform_a, mut impulse_a, effect_a), (entity_b, transform_b, mut impulse_b, effect_b)],
    ) = combinations.fetch_next()
    {
        let impulse = calculate_magnetic_impulse(transform_a, transform_b, effect_a);

        if let Some(impulse) = impulse {
            impulse_b.impulse = impulse;
            impulse_a.impulse -= impulse;
        }

        let impulse = calculate_magnetic_impulse(transform_b, transform_a, effect_b);
        if let Some(impulse) = impulse {
            impulse_a.impulse = impulse;
            impulse_b.impulse -= impulse;
        }
    }
}

pub fn calculate_magnetic_impulse(
    magnet_transform: &Transform,
    block_transform: &Transform,
    effect: &MagneticEffect,
) -> Option<Vec2> {
    let distance = magnet_transform
        .translation
        .distance(block_transform.translation);
    if distance < effect.range {
        let direction =
            (magnet_transform.translation.xy() - block_transform.translation.xy()).normalize();

        // max force at 0 distance, 0 force at max distance
        let force = (effect.range - distance) / effect.range * effect.force;

        let force = direction * force;
        Some(force)
    } else {
        None
    }
}
