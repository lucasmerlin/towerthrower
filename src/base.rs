use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::prelude::{Friction, RigidBody, Velocity};

use crate::cursor_system::CursorCoords;
use crate::level::{Level, LevelLifecycle};
use crate::state::LevelState;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup_base);
    }
}

#[derive(Component)]
pub struct Base;

pub fn setup_base(mut commands: Commands, mut assets: ResMut<AssetServer>, mut level: Res<Level>) {
    let height = 20.0;
    let width = level.bases[0].width;

    for base in level.bases {
        commands
            .spawn((
                Base,
                LevelLifecycle,
                SpatialBundle::from(
                    Transform::from_translation(Vec3::from((
                        base.translation + Vec2::new(0.0, height / 2.0),
                        0.0,
                    )))
                    .with_rotation(Quat::from_rotation_z(base.rotation)),
                ),
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(width / 2.0, height / 2.0),
                Friction::coefficient(0.5),
                Velocity::linear(Vec2::new(0.0, 0.0)),
            ))
            .with_children(|parent| {
                parent.spawn(SpriteBundle {
                    // transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    texture: assets.load("fortress.png"),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.1)),
                    ..Default::default()
                });
            });
    }
}

// pub fn base_position(mycoords: Res<MyWorldCoords>, mut query: Query<&mut Transform, With<Base>>) {
//     for mut transform in query.iter_mut() {
//         transform.translation.x = mycoords.0.x;
//         //transform.translation.y = mycoords.0.y;
//     }
// }

pub fn base_position(key_code: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Base>>) {
    for mut velocity in query.iter_mut() {
        let max_velocity = 100.0;
        let increment = 1.0;
        let decrement = 1.0;

        if key_code.pressed(KeyCode::Left) {
            velocity.linvel.x -= increment;
            if velocity.linvel.x < -max_velocity {
                velocity.linvel.x = -max_velocity;
            }
        } else if key_code.pressed(KeyCode::Right) {
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
            if velocity.linvel.x.abs() < 1.0 {
                velocity.linvel.x = 0.0;
            }
        }

        //transform.translation.y = mycoords.0.y;
    }
}
