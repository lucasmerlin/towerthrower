use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::block::{Aiming, Block, BlockType, Falling};
use crate::camera_movement::CameraMovement;
use crate::cursor_system::CursorCoords;

pub struct ThrowPlugin;

impl Plugin for ThrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                simulate_throw_system,
                create_aiming_block,
                fill_throw_queue,
                update_aim_system,
                update_aim_from_mouse_position,
                throw_system,
            ),
        )
        .init_resource::<Aim>()
        .init_resource::<ThrowQueue>();
    }
}

#[derive(Resource, Debug)]
pub struct Aim {
    pub direction: Vec2,
    pub force: f32,
    pub rotation: f32,
}
impl Default for Aim {
    fn default() -> Self {
        Self {
            direction: Vec2::from_angle(PI / 1.5),
            force: 500.0,
            rotation: 0.1,
        }
    }
}

impl Aim {
    fn velocity(&self) -> Velocity {
        Velocity {
            linvel: self.direction * self.force,
            angvel: self.rotation,
        }
    }
}

#[derive(Resource, Debug)]
pub struct ThrowQueue {
    pub target_length: usize,
    pub queue: Vec<BlockType>,
}
impl Default for ThrowQueue {
    fn default() -> Self {
        Self {
            target_length: 3,
            queue: vec![],
        }
    }
}

#[derive(Component, Debug)]
pub struct TargetIndicator;

pub fn simulate_throw_system(
    mut commands: Commands,
    aim: Res<Aim>,
    aimed_block: Query<(Entity, &Collider, &Transform), (With<Block>, With<Aiming>)>,
    rapier_context: Res<RapierContext>,
    old_target_indicators: Query<Entity, With<TargetIndicator>>,
    mut assets: ResMut<AssetServer>,
) {
    if let Ok((aimed, aimed_collider, aimed_transform)) = aimed_block.get_single() {
        let mut t = 0.0;
        let dt = 0.001;

        let shape = aimed_collider.clone();
        let mut transform = aimed_transform.clone();
        let mut velocity = aim.velocity();
        let mut acceleration = Vec2::Y * -9.81 * 100.0;

        let mut hit = false;

        let mut steps = vec![];

        while t < 2.0 {
            let mut intersection = rapier_context.intersection_with_shape(
                transform.translation.xy(),
                transform.rotation.angle_between(Quat::from_rotation_z(0.0)),
                &shape,
                QueryFilter::default()
                    .exclude_collider(aimed)
                    .exclude_sensors(),
            );

            if intersection.is_some() {
                dbg!(intersection, aimed);
                hit = true;
                break;
            }

            transform.translation += Vec3::from((velocity.linvel * dt, 0.0));
            transform.rotation = transform.rotation * Quat::from_rotation_z(velocity.angvel * dt);
            velocity.linvel += acceleration * dt;

            steps.push(transform.clone());

            t += dt;
        }

        println!("transform: {:?}", transform);
        println!("velocity: {:?}", velocity);
        println!("acceleration: {:?}", acceleration);
        println!("hit: {:?}", hit);
        //rapier_context.intersection_with_shape()

        // remove previous target indicators
        for entity in old_target_indicators.iter() {
            commands.entity(entity).despawn_recursive();
        }

        for step in steps {
            commands.spawn((
                SpriteBundle {
                    transform: step,
                    texture: assets.load("aim.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(2.0, 2.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                TargetIndicator,
                Sensor,
            ));
        }

        // spawn new target indicator
        commands.spawn((
            SpriteBundle {
                transform,
                texture: assets.load("aim.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            TargetIndicator,
            shape,
            Sensor,
        ));
    }
}

pub fn create_aiming_block(
    mut commands: Commands,
    mut throw_queue: ResMut<ThrowQueue>,
    mut query: Query<(Entity), With<Aiming>>,
    mut assets: ResMut<AssetServer>,
    mut camera_movement: ResMut<CameraMovement>,
) {
    if query.iter().count() == 0 {
        if let Some(block_type) = throw_queue.queue.pop() {
            Block::spawn(
                &mut commands,
                block_type,
                Vec2::new(400.0, -299.0 + camera_movement.height),
                &mut assets,
            );
        }
    }
}

pub fn fill_throw_queue(mut throw_queue: ResMut<ThrowQueue>) {
    while throw_queue.queue.len() < throw_queue.target_length {
        throw_queue.queue.push(BlockType::random());
    }
}

pub fn update_aim_system(
    mut aim: ResMut<Aim>,
    mut query: Query<&mut Transform, With<Aiming>>,
    key_code: Res<Input<KeyCode>>,
) {
    if key_code.pressed(KeyCode::Left) {
        aim.direction = aim.direction.rotate(Vec2::from_angle(0.005));
    } else if key_code.pressed(KeyCode::Right) {
        aim.direction = aim.direction.rotate(Vec2::from_angle(-0.005));
    }

    if key_code.pressed(KeyCode::Up) {
        aim.force += 2.0;
    } else if key_code.pressed(KeyCode::Down) {
        aim.force -= 2.0;
    }

    if key_code.pressed(KeyCode::Q) {
        aim.rotation += 0.0125;
    } else if key_code.pressed(KeyCode::E) {
        aim.rotation -= 0.0125;
    }
}

pub fn update_aim_from_mouse_position(
    mut aim: ResMut<Aim>,
    mut query: Query<&mut Transform, With<Aiming>>,
    mouse_position: Res<CursorCoords>,
) {
    let mouse_position = mouse_position.0;
    let mut query = query.iter_mut();
    if let Some(mut transform) = query.next() {
        let force =
            calculate_velocity_for_throw(transform.translation.xy(), mouse_position, aim.direction);

        aim.force = force;
    }
}

pub fn calculate_velocity_for_throw(source: Vec2, target: Vec2, launch_direction: Vec2) -> f32 {
    let displacement = target - source;
    let horizontal_displacement = displacement.x;

    // Assuming time is constant (you may need to experiment with this value)
    let time = 1.0;

    let horizontal_velocity = horizontal_displacement / time;

    let vertical_displacement = displacement.y;
    let gravity = 9.8; // Adjust this value based on your game's requirements

    let vertical_velocity = (vertical_displacement + 0.5 * gravity * time.powi(2)) / time;

    let mut initial_velocity = Vec2::new(horizontal_velocity, vertical_velocity);

    // Normalize launch direction
    let normalized_launch_direction = launch_direction.normalize();
    // Scale initial velocity by launch direction magnitude
    initial_velocity *= normalized_launch_direction.length();

    // Return the magnitude of the adjusted initial velocity
    return initial_velocity.length();
}

pub fn throw_system(
    mut commands: Commands,
    mut input: ResMut<Input<KeyCode>>,
    mut aim: ResMut<Aim>,
    mut query: Query<(Entity), With<Aiming>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for entity in query.iter_mut() {
            commands
                .entity(entity)
                .remove::<Aiming>()
                .remove::<Sensor>()
                .insert((Falling, RigidBody::Dynamic, aim.velocity()));
        }
    }
}
