use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;

use crate::consts::{foreground_collision_groups, RAIN_COLLISION_GROUP};
use crate::level::{Level, LevelLifecycle};
use crate::state::LevelState;
use crate::HORIZONTAL_VIEWPORT_SIZE;

pub struct RainPlugin;

impl Plugin for RainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup_rain);
        app.add_systems(
            Update,
            (
                spawn_rain,
                rain_collision_system,
                darken_sprite_on_rain_system,
            ),
        )
        .init_resource::<RainSpawner>();
    }
}

#[derive(Component, Debug)]
pub struct Rain;
#[derive(Component, Debug)]
pub struct Splash;

#[derive(Resource, Debug)]
pub struct RainSpawner {
    pub timer: Timer,
}

#[derive(Component, Debug)]
pub struct DarkenSpriteOnRain(pub f32);

impl Default for RainSpawner {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.01, TimerMode::Repeating),
        }
    }
}

pub fn setup_rain(mut commands: Commands, assets: Res<AssetServer>, level: Res<Level>) {
    if level.rain.is_some() {
        commands.spawn((
            LevelLifecycle,
            AudioBundle {
                source: assets.load("sounds/rain.wav"),
                settings: PlaybackSettings::LOOP,
            },
        ));
    }
}

pub fn spawn_rain(
    mut commands: Commands,
    mut spawner: ResMut<RainSpawner>,
    time: Res<Time>,
    level: Res<Level>,
) {
    if let Some(rain_amount) = level.rain {
        if spawner.timer.tick(time.delta()).just_finished() {
            for i in 0..rain_amount {
                let x = rand::random::<f32>() * HORIZONTAL_VIEWPORT_SIZE
                    - HORIZONTAL_VIEWPORT_SIZE / 2.0;
                let y = 30.0 + random::<f32>() * 30.0;

                let collision_group = if random() {
                    CollisionGroups {
                        memberships: RAIN_COLLISION_GROUP,
                        filters: Group::ALL & !RAIN_COLLISION_GROUP,
                    }
                } else {
                    CollisionGroups {
                        memberships: RAIN_COLLISION_GROUP,
                        filters: foreground_collision_groups() & !RAIN_COLLISION_GROUP,
                    }
                };

                commands.spawn((
                    Rain,
                    Collider::ball(0.05),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba_u8(220, 220, 255, 150),
                            custom_size: Some(Vec2::new(0.04, 0.4)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(x, y, 0.0),
                        ..Default::default()
                    },
                    RigidBody::KinematicVelocityBased,
                    Velocity::linear(Vec2::new(0.0, -15.0 + random::<f32>() * -5.0)),
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::DYNAMIC_KINEMATIC
                        | ActiveCollisionTypes::KINEMATIC_STATIC
                        | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
                    Sensor,
                    collision_group,
                ));
            }
        }
    }
}

pub fn rain_collision_system(
    mut commands: Commands,
    mut collisions: EventReader<CollisionEvent>,
    rain_query: Query<(&Transform), With<Rain>>,
    splash_query: Query<(&Transform), With<Splash>>,
) {
    for event in collisions.read() {
        match event {
            CollisionEvent::Started(collider1, collider2, _) => {
                [(collider1, collider2), (collider2, collider1)]
                    .into_iter()
                    .for_each(|(rain_entity, other_entity)| {
                        if let Ok(transform) = rain_query.get(*rain_entity) {
                            commands.entity(*rain_entity).despawn_recursive();

                            let mut transform = transform.clone();
                            transform.translation.y += 0.1 * random::<f32>() + 0.1;

                            for i in -1..=1 {
                                commands.spawn((
                                    Collider::ball(0.025),
                                    Splash,
                                    SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::rgba_u8(200, 200, 255, 200),
                                            custom_size: Some(Vec2::new(0.05, 0.05)),
                                            ..Default::default()
                                        },
                                        transform,
                                        ..Default::default()
                                    },
                                    RigidBody::Dynamic,
                                    Velocity::linear(Vec2::new(i as f32 * 3.0, 3.0)),
                                    ActiveEvents::COLLISION_EVENTS,
                                    Sensor,
                                    CollisionGroups {
                                        memberships: RAIN_COLLISION_GROUP,
                                        filters: Group::ALL & !RAIN_COLLISION_GROUP,
                                    },
                                ));
                            }
                        }
                        if let Ok(transform) = splash_query.get(*other_entity) {
                            commands.entity(*other_entity).despawn_recursive();
                        }
                    });
            }
            _ => {}
        }
    }
}

pub fn darken_sprite_on_rain_system(
    mut query: Query<(&mut Sprite, &DarkenSpriteOnRain)>,
    level: Res<Level>,
) {
    query
        .iter_mut()
        .for_each(|(mut sprite, DarkenSpriteOnRain(darken))| {
            let color = if level.rain.is_some() {
                Color::rgb(0.7 / darken, 0.7 / darken, 0.7 / darken)
            } else {
                Color::WHITE
            };
            sprite.color = color;
        });
}
