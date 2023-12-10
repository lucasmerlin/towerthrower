use crate::level::LevelLifecycle;
use crate::state::LevelState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LaunchPlatformPlugin;

impl Plugin for LaunchPlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), (spawn_launch_platform_system))
            .add_systems(
                Update,
                (launch_platform_control_system.run_if(in_state(LevelState::Playing))),
            );
    }
}

#[derive(Component, Debug, Default)]
pub struct LaunchPlatform;

pub fn spawn_launch_platform_system(mut commands: Commands, mut assets: ResMut<AssetServer>) {
    let collider = Collider::cuboid(100.0, 10.0);

    commands
        .spawn((
            LaunchPlatform,
            LevelLifecycle,
            SpatialBundle::from(Transform::from_translation(Vec3::new(10.0, 10.0, 0.0))),
            RigidBody::KinematicVelocityBased,
            Velocity::zero(),
            //collider,
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: assets.load("rocket.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn launch_platform_control_system(
    key_code: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<LaunchPlatform>>,
) {
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
