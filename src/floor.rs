use crate::block::DestroyBlockOnContact;
use crate::level::LevelLifecycle;
use crate::state::LevelState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup_floor);
    }
}

#[derive(Component, Debug)]
pub struct Floor;

pub const FLOOR_COLLISION_GROUP: Group = Group::GROUP_2;

pub fn setup_floor(mut commands: Commands) {
    let width = 10000.0;
    let height = 1000.0;

    let collider = Collider::cuboid(width / 2.0, height / 2.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -height / 2.0 + -100.0, 0.0),
            ..Default::default()
        },
        Floor,
        LevelLifecycle,
        DestroyBlockOnContact,
        RigidBody::Fixed,
        collider,
        CollisionGroups {
            memberships: FLOOR_COLLISION_GROUP,
            ..Default::default()
        },
    ));
}
