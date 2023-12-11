use crate::block::{Block, BlockType, DestroyBlockOnContact, BLOCK_SIZE};
use crate::consts::{BLOCK_COLLISION_GROUP, DEBRIS_COLLISION_GROUP};
use crate::floor::Floor;
use crate::level::{LevelLifecycle, UpdateLevelStats};
use bevy::audio::{Volume, VolumeLevel};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{random, thread_rng, Rng};

pub struct DebrisPlugin;

impl Plugin for DebrisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (block_to_debris_system));
    }
}

#[derive(Component, Debug)]
pub struct Debris(Timer, Timer);

impl Default for Debris {
    fn default() -> Self {
        let duration = random::<f32>() * 3.0 + 5.0;
        Self(
            Timer::from_seconds(duration, TimerMode::Once),
            Timer::from_seconds(duration + 0.5, TimerMode::Once),
        )
    }
}

pub fn block_to_debris_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut floor_query: Query<&mut DestroyBlockOnContact>,
    mut block_query: Query<(Entity, &Block, &Transform, &Velocity)>,
    mut update_level_stats_events: EventWriter<UpdateLevelStats>,
    assets: Res<AssetServer>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                [(a, b), (b, a)].into_iter().for_each(|(floor, block)| {
                    if floor_query.get(*floor).is_err() {
                        return;
                    }

                    if let Ok((entity, block, transform, velocity)) = block_query.get_mut(*block) {
                        commands.entity(entity).despawn_recursive();

                        for pos in block.block_type.get_shape() {
                            let pos = transform
                                .compute_matrix()
                                .transform_point(Vec3::from((pos, 0.0)));

                            commands.spawn((
                                Debris::default(),
                                LevelLifecycle,
                                SpriteBundle {
                                    transform: Transform::from_xyz(pos.x, pos.y, 0.0)
                                        .with_rotation(transform.rotation),

                                    texture: assets.load(format!(
                                        "debris/debris_{}.png",
                                        thread_rng().gen_range(1..=3)
                                    )),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                RigidBody::Dynamic,
                                Collider::cuboid(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0),
                                Friction::coefficient(0.5),
                                Velocity {
                                    linvel: velocity.linvel,
                                    angvel: velocity.angvel,
                                },
                                ExternalImpulse::default(),
                                //Dominance::group(-1),
                                CollisionGroups {
                                    memberships: DEBRIS_COLLISION_GROUP,
                                    filters: {
                                        let mut group = Group::ALL;
                                        group.remove(BLOCK_COLLISION_GROUP);
                                        group
                                    },
                                },
                            ));
                        }

                        update_level_stats_events.send(UpdateLevelStats::BlockDestroyed);

                        // Since O always has a window, we play the glass sound
                        let sound = if block.block_type == BlockType::O {
                            "sounds/glass.wav"
                        } else {
                            "sounds/debris.wav"
                        };
                        commands.spawn(AudioBundle {
                            source: assets.load(sound),
                            settings: PlaybackSettings {
                                volume: Volume::Relative(VolumeLevel::new(0.7)),
                                ..PlaybackSettings::DESPAWN
                            },
                        });
                    }
                })
            }
            CollisionEvent::Stopped(a, b, _) => {}
        }
    }
}

pub fn despawn_debris_system(
    mut commands: Commands,
    mut debris_query: Query<(Entity, &mut Debris, &mut SolverGroups)>,
    mut timer: ResMut<Time>,
) {
    for (entity, mut debris, mut groups) in debris_query.iter_mut() {
        debris.0.tick(timer.delta());
        debris.1.tick(timer.delta());
        if debris.0.finished() {
            groups.filters = Group::empty();
        }
        if debris.1.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
