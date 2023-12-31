use crate::{ASSET_SCALE, CAR_SCALE, FLOOR_HEIGHT, HORIZONTAL_VIEWPORT_SIZE};
use bevy::audio::{Volume, VolumeLevel};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::debris::Debris;
use crate::level::LevelLifecycle;

pub struct DebrisCleanerPlugin;

impl Plugin for DebrisCleanerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_debris_cleaner,
                debris_cleaner_attraction,
                debris_cleaner_collision,
            ),
        );
    }
}

#[derive(Component, Debug)]
pub struct DebrisCleaner;

pub const DEBRIS_CLEANER_THRESHOLD: usize = 30;
pub const DEBRIS_CLEANER_RANGE: f32 = 8.0;
pub const DEBRIS_CLEANER_FORCE: f32 = 2.0;

pub fn spawn_debris_cleaner(
    mut commands: Commands,
    mut debris_query: Query<Entity, With<Debris>>,
    mut debris_cleaner_query: Query<Entity, With<DebrisCleaner>>,
    mut assets: ResMut<AssetServer>,
) {
    let res_w = 1395.0;
    let res_h = 671.0;

    let size = Vec2::new(res_w, res_h) * ASSET_SCALE * CAR_SCALE * 0.8;

    let count = debris_query.iter_mut().count();

    let mut debris_cleaner_count = debris_cleaner_query.iter_mut().count();

    if count > DEBRIS_CLEANER_THRESHOLD && debris_cleaner_count == 0 {
        commands.spawn((
            DebrisCleaner,
            LevelLifecycle,
            SpriteBundle {
                transform: Transform::from_xyz(HORIZONTAL_VIEWPORT_SIZE, FLOOR_HEIGHT + 1.0, 0.1)
                    .with_rotation(Quat::from_rotation_z(0.0)),
                texture: assets.load("cars/garbage_collector.png"),
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::KinematicVelocityBased,
            Collider::cuboid(size.x / 3.0, size.y / 2.0),
            Friction::coefficient(0.5),
            Velocity::linear(Vec2::new(-4.0, 0.0)),
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
        ));
    }
}

// The debris cleaner attracts all debris within it's range and despawns it on collision
pub fn debris_cleaner_attraction(
    mut commands: Commands,
    mut debris_cleaner_query: Query<(Entity, &Transform), With<DebrisCleaner>>,
    mut debris_query: Query<(Entity, &Transform, &mut ExternalImpulse), With<Debris>>,
) {
    for (debris_cleaner_entity, debris_cleaner_transform) in debris_cleaner_query.iter_mut() {
        for (debris_entity, debris_transform, mut debris_impulse) in debris_query.iter_mut() {
            let distance = debris_transform
                .translation
                .distance(debris_cleaner_transform.translation);

            if distance < DEBRIS_CLEANER_RANGE {
                let direction = debris_cleaner_transform.translation - debris_transform.translation;
                let direction = direction.normalize();

                let force =
                    direction * DEBRIS_CLEANER_FORCE * (1.0 - distance / DEBRIS_CLEANER_RANGE);

                debris_impulse.impulse += force.xy();
            }
        }
    }
}

pub fn debris_cleaner_collision(
    mut commands: Commands,
    mut debris_cleaner_query: Query<Entity, With<DebrisCleaner>>,
    mut debris_query: Query<Entity, With<Debris>>,
    mut debris_cleaner_collision_events: EventReader<CollisionEvent>,
    mut assets: ResMut<AssetServer>,
) {
    for event in debris_cleaner_collision_events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                [(a, b), (b, a)]
                    .into_iter()
                    .for_each(|(debris_cleaner, debris)| {
                        if debris_cleaner_query.get(*debris_cleaner).is_err() {
                            return;
                        }

                        if let Ok(debris) = debris_query.get_mut(*debris) {
                            commands.entity(debris).despawn_recursive();
                            commands.spawn(
                                (AudioBundle {
                                    source: assets.load("sounds/whoosh.wav"),
                                    settings: PlaybackSettings {
                                        volume: Volume::Relative(VolumeLevel::new(0.5)),
                                        ..PlaybackSettings::DESPAWN
                                    },
                                }),
                            );
                        }
                    });
            }
            _ => {}
        }
    }
}
