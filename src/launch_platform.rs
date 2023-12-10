use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::prelude::*;

use crate::level::{LaunchPlatformKind, Level, LevelLifecycle};
use crate::state::LevelState;
use crate::throw::Aim;
use crate::ASSET_SCALE;

pub struct LaunchPlatformPlugin;

impl Plugin for LaunchPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), (spawn_launch_platform_system))
            .add_systems(
                Update,
                ((barrel_rotation_system, launch_platform_control_system)
                    .run_if(in_state(LevelState::Playing)),),
            );
    }
}

#[derive(Component, Debug, Default)]
pub struct LaunchPlatform;

#[derive(Component, Debug)]
pub struct Barrel;

pub fn spawn_launch_platform_system(
    mut commands: Commands,
    mut assets: ResMut<AssetServer>,
    level: Res<Level>,
) {
    let collider = Collider::cuboid(100.0, 10.0);

    let res_w = 370.0;
    let res_h = 255.0;
    let res_h_on = 490.0;

    let size = Vec2::new(res_w * ASSET_SCALE, res_h * ASSET_SCALE);

    let barrel_res_w = 250.0;
    let barrel_res_h = 400.0;

    let barrel_size = Vec2::new(barrel_res_w * ASSET_SCALE, barrel_res_h * ASSET_SCALE);

    commands
        .spawn((
            LaunchPlatform,
            LevelLifecycle,
            SpatialBundle::from(Transform::from_translation(Vec3::from((
                level.launch_platform.translation,
                0.0,
            )))),
            RigidBody::KinematicVelocityBased,
            Velocity::zero(),
            //collider,
        ))
        .with_children(|parent| {
            if let LaunchPlatformKind::Static = level.launch_platform.kind {
                parent.spawn(SpriteBundle {
                    texture: assets.load("cannon/off.png"),
                    sprite: Sprite {
                        custom_size: Some(size),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                });
            } else {
                parent.spawn((SpriteBundle {
                    texture: assets.load("cannon/on.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(res_w * ASSET_SCALE, res_h_on * ASSET_SCALE)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        (size.y - (res_h_on * ASSET_SCALE)) / 2.0,
                        0.0,
                    )),
                    ..Default::default()
                },));
            }

            parent.spawn((
                Barrel,
                SpriteBundle {
                    texture: assets.load("cannon/barrel.png"),
                    sprite: Sprite {
                        custom_size: Some(barrel_size),
                        anchor: Anchor::Custom(Vec2::new(0.0, -0.35)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    ..Default::default()
                },
            ));
        });
}

pub fn launch_platform_control_system(
    key_code: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<LaunchPlatform>>,
    level: Res<Level>,
) {
    if let LaunchPlatformKind::Static = level.launch_platform.kind {
        return;
    }
    for mut velocity in query.iter_mut() {
        let max_velocity = 25.0;
        let increment = 0.5;
        let decrement = 0.5;

        /// WASD for launch platform
        if key_code.pressed(KeyCode::A) {
            velocity.linvel.x -= increment;
            if velocity.linvel.x < -max_velocity {
                velocity.linvel.x = -max_velocity;
            }
        } else if key_code.pressed(KeyCode::D) {
            velocity.linvel.x += increment;
            if velocity.linvel.x > max_velocity {
                velocity.linvel.x = max_velocity;
            }
        } else {
            if velocity.linvel.x > 0.0 {
                velocity.linvel.x -= decrement
            } else if velocity.linvel.x < 0.0 {
                velocity.linvel.x += decrement
            }

            if velocity.linvel.x < decrement && velocity.linvel.x > -decrement {
                velocity.linvel.x = 0.0;
            }
        }

        if key_code.pressed(KeyCode::W) {
            velocity.linvel.y += increment;
            if velocity.linvel.y > max_velocity {
                velocity.linvel.y = max_velocity;
            }
        } else if key_code.pressed(KeyCode::S) {
            velocity.linvel.y -= increment;
            if velocity.linvel.y < -max_velocity {
                velocity.linvel.y = -max_velocity;
            }
        } else {
            if velocity.linvel.y > 0.0 {
                velocity.linvel.y -= decrement
            } else if velocity.linvel.y < 0.0 {
                velocity.linvel.y += decrement
            }

            if velocity.linvel.y < decrement && velocity.linvel.y > -decrement {
                velocity.linvel.y = 0.0;
            }
        }
    }
}

pub fn barrel_rotation_system(aim: Res<Aim>, mut query: Query<&mut Transform, With<Barrel>>) {
    let barrel = query.get_single_mut();

    if let Ok(mut barrel) = barrel {
        barrel.rotation = Quat::from_rotation_z(
            -aim.barrel_direction
                .unwrap_or(Vec2::Y)
                .angle_between(Vec2::Y),
        );
    }
}
