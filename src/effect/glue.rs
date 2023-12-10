use std::mem;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::block::{Block, CaughtBlock, FallingBlockCollision};
use crate::effect::glue_texture;

pub struct GluePlugin;

impl Plugin for GluePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (trigger_glue_phase_system, collect_glue_list_system),
        );
    }
}

#[derive(Component, Debug, Default)]
pub struct GlueEffect(pub GlueEffectPhase);

#[derive(Debug, Clone)]
pub enum GlueEffectPhase {
    Gluing { targets: Vec<Entity> },
    Glued,
}

impl Default for GlueEffectPhase {
    fn default() -> Self {
        Self::Gluing { targets: vec![] }
    }
}

#[derive(Component, Debug)]
pub struct GlueJoint;

pub fn collect_glue_list_system(
    mut commands: Commands,
    mut event_reader: EventReader<FallingBlockCollision>,
    mut query: Query<(Entity, &mut GlueEffect)>,
    hit_query: Query<(Entity, &Block)>,
    assets: Res<AssetServer>,
) {
    for event in event_reader.read() {
        let FallingBlockCollision { falling, hit } = event;

        if let Ok((entity, mut glue_effect)) = query.get_mut(*falling) {
            if let GlueEffectPhase::Gluing { targets } = &mut glue_effect.0 {
                if !targets.contains(hit) {
                    targets.push(*hit);

                    if let Ok((hit, block)) = hit_query.get(*hit) {
                        commands.entity(hit).with_children(|parent| {
                            parent.spawn(
                                (SpriteBundle {
                                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                    texture: assets.load(glue_texture(block.block_type)),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(
                                            block.block_type.width(),
                                            block.block_type.height(),
                                        )),
                                        color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }),
                            );
                        });
                    }
                }
            }
        }
    }
}

pub fn trigger_glue_phase_system(
    mut commands: Commands,
    mut event_reader: EventReader<CaughtBlock>,
    mut query: Query<(Entity, &mut GlueEffect, &Transform)>,
    mut target_query: Query<(Entity, &Transform)>,
) {
    for event in event_reader.read() {
        let CaughtBlock { entity } = event;

        if let Ok((entity, mut glue_effect, transform)) = query.get_mut(*entity) {
            if let GlueEffectPhase::Gluing { targets } = &glue_effect.0 {
                dbg!("Glueing to entity", entity, &targets);
                for target in targets {
                    if let Ok((target, target_transform)) = target_query.get(*target) {
                        let base_transform = target_transform;
                        let falling_transform = transform;

                        let base_translation = base_transform.translation.xy();
                        let falling_translation = falling_transform.translation.xy();

                        let base_rotation = base_transform
                            .rotation
                            .angle_between(Quat::from_rotation_z(0.0));
                        let falling_rotation = falling_transform
                            .rotation
                            .angle_between(Quat::from_rotation_z(0.0));

                        let offset_global = falling_translation - base_translation;
                        let offset_local = base_transform.rotation.inverse()
                            * (falling_transform.translation - base_transform.translation);

                        let base_anchor = offset_local.xy();
                        let falling_anchor = Vec2::ZERO;

                        let base_angle = 0.0;

                        // now calculate the falling angle.

                        let base_fwd = (base_transform.rotation * Vec3::X).xy();
                        let base_right = (base_transform.rotation * Vec3::Y).xy();

                        let falling_fwd = (falling_transform.rotation * Vec3::X).xy();
                        let falling_right = (falling_transform.rotation * Vec3::Y).xy();

                        //let base_angle = base_fwd.angle_between(falling_fwd);
                        let falling_angle = falling_fwd.angle_between(base_fwd);

                        let mut joint = FixedJointBuilder::new()
                            .local_anchor1(falling_anchor)
                            .local_anchor2(base_anchor)
                            .local_basis1(falling_angle)
                            .local_basis2(base_angle)
                            .build();
                        joint.set_contacts_enabled(false);

                        commands.entity(target).with_children(|parent| {
                            parent.spawn((GlueJoint, ImpulseJoint::new(entity, joint)));
                        });
                    }
                }
            }
        }
    }
}
