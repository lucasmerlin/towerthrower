use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::debris::Debris;

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
pub const DEBRIS_CLEANER_RANGE: f32 = 300.0;
pub const DEBRIS_CLEANER_FORCE: f32 = 20.0;

pub fn spawn_debris_cleaner(
    mut commands: Commands,
    mut debris_query: Query<Entity, With<Debris>>,
    mut debris_cleaner_query: Query<Entity, With<DebrisCleaner>>,
) {
    let count = debris_query.iter_mut().count();

    let mut debris_cleaner_count = debris_cleaner_query.iter_mut().count();

    if count > DEBRIS_CLEANER_THRESHOLD && debris_cleaner_count == 0 {
        commands.spawn((
            DebrisCleaner,
            SpatialBundle::from(
                Transform::from_xyz(1000.0, -50.0, 0.0).with_rotation(Quat::from_rotation_z(0.0)),
            ),
            RigidBody::KinematicVelocityBased,
            Collider::cuboid(100.0, 50.0),
            Friction::coefficient(0.5),
            Velocity::linear(Vec2::new(-40.0, 0.0)),
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
                                    source: assets.load("sounds/debris_collect.wav"),
                                    settings: PlaybackSettings::DESPAWN,
                                }),
                            );
                        }
                    });
            }
            _ => {}
        }
    }
}
