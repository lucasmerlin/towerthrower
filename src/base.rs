use crate::block::BLOCK_SIZE;
use crate::collision_sounds::CollisionSound;
use crate::consts::BASE_COLLISION_GROUP;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::plugin::systems::apply_scale;
use bevy_rapier2d::prelude::{CollisionGroups, Friction, Group, RigidBody, Velocity};

use crate::cursor_system::CursorCoords;
use crate::environment::rain::DarkenSpriteOnRain;
use crate::level::{Level, LevelLifecycle};
use crate::state::LevelState;
use crate::HORIZONTAL_VIEWPORT_SIZE;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup_base);
    }
}

#[derive(Debug, Clone)]
pub enum BaseType {
    T2,
    T3,
    T4,
    T7,
    T9,
}

impl BaseType {
    pub fn name(&self) -> &str {
        match self {
            BaseType::T2 => "t-2",
            BaseType::T3 => "t-3",
            BaseType::T4 => "t-4",
            BaseType::T7 => "t-7",
            BaseType::T9 => "t-9",
        }
    }

    pub fn width(&self) -> f32 {
        let image_width = self.image_width();
        let _4k_width = 3840.0;
        let scale = HORIZONTAL_VIEWPORT_SIZE / _4k_width;
        image_width * scale
    }

    pub fn image_width(&self) -> f32 {
        match self {
            BaseType::T2 => 466.0,
            BaseType::T3 => 636.0,
            BaseType::T4 => 762.0,
            BaseType::T7 => 1324.0,
            BaseType::T9 => 1652.0,
        }
    }

    pub fn asset(&self) -> String {
        format!("bases/{}.png", self.name())
    }
}

#[derive(Component)]
pub struct Base;

pub fn setup_base(mut commands: Commands, mut assets: ResMut<AssetServer>, mut level: Res<Level>) {
    let height = BLOCK_SIZE;

    // Since the spot in the bg image is not centered, we need to offset the base a bit
    let additional_transform = Vec2::new(0.5, 0.0);

    for base in level.bases {
        let width = base.base_type.width();

        let image_width = base.base_type.image_width();

        let image_scale = width / image_width;

        let texture = assets.load(base.base_type.asset());

        commands
            .spawn((
                CollisionSound::default(),
                Base,
                LevelLifecycle,
                SpatialBundle::from(
                    Transform::from_translation(Vec3::from((
                        base.translation + additional_transform + Vec2::new(0.0, height / 2.0),
                        0.0,
                    )))
                    .with_rotation(Quat::from_rotation_z(base.rotation)),
                ),
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(width / 2.0, height / 2.0),
                Friction::coefficient(level.friction),
                Velocity::linear(Vec2::new(0.0, 0.0)),
                CollisionGroups {
                    filters: Group::ALL,
                    memberships: BASE_COLLISION_GROUP,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        texture,
                        transform: Transform::from_xyz(0.0, height / 2.0, -7.0)
                            .with_scale(Vec3::splat(image_scale)),
                        sprite: Sprite {
                            anchor: Anchor::TopCenter,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    DarkenSpriteOnRain(1.0),
                ));
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
