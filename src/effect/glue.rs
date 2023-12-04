use std::mem;

use crate::block::CaughtBlock;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GluePlugin;

impl Plugin for GluePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (trigger_glue_phase_system, glue_phase_timer_system));
    }
}

#[derive(Component, Debug, Default)]
pub struct GlueEffect(pub GlueEffectPhase);

#[derive(Debug, Clone, Default)]
pub enum GlueEffectPhase {
    #[default]
    Idle,
    Gluing {
        targets: Vec<Entity>,
        timer: Timer,
    },
    Glued,
}

#[derive(Component, Debug)]
pub struct GlueJoint;

// on first contact the glue.png timer starts and any additional contact will add the entity to the glue.png list
// once the timer is done all entities in the glue.png list will be glued together with a joint

// query for contact events
pub fn trigger_glue_phase_system(
    mut commands: Commands,
    mut event_reader: EventReader<CaughtBlock>,
    mut query: Query<(Entity, &mut GlueEffect)>,
) {
    for event in event_reader.read() {
        let CaughtBlock { entity, caught_by } = event;

        if let Ok((entity, mut glue_effect)) = query.get_mut(*entity) {
            let old = mem::take(&mut glue_effect.0);
            glue_effect.0 = match old {
                GlueEffectPhase::Idle => GlueEffectPhase::Gluing {
                    targets: vec![*caught_by],
                    timer: Timer::from_seconds(0.2, TimerMode::Once),
                },
                GlueEffectPhase::Gluing { mut targets, timer } => {
                    if !targets.contains(caught_by) {
                        targets.push(*caught_by);
                    }
                    GlueEffectPhase::Gluing { targets, timer }
                }
                GlueEffectPhase::Glued => GlueEffectPhase::Glued,
            }
        }
    }
}

pub fn glue_phase_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GlueEffect, &Transform)>,
    mut target_query: Query<(Entity, &Transform)>,
) {
    for (entity, mut glue_effect, transform) in query.iter_mut() {
        let old = mem::take(&mut glue_effect.0);
        glue_effect.0 = match old {
            GlueEffectPhase::Idle => GlueEffectPhase::Idle,
            GlueEffectPhase::Gluing { targets, mut timer } => {
                timer.tick(time.delta());
                if timer.finished() {
                    dbg!("Glueing to entity", entity, &targets);
                    for target in targets {
                        if let Ok((target, target_transform)) = target_query.get(target) {
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

                    GlueEffectPhase::Glued
                } else {
                    GlueEffectPhase::Gluing { targets, timer }
                }
            }
            GlueEffectPhase::Glued => GlueEffectPhase::Glued,
        }
    }
}
