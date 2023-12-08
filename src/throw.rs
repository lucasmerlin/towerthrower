use std::f32::consts::PI;

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::block::{Aiming, Block, BlockType, Falling};
use crate::camera_movement::CameraMovement;
use crate::cursor_system::CursorCoords;
use crate::effect::magnetic::{calculate_magnetic_impulse, MagneticEffect};
use crate::launch_platform::LaunchPlatform;
use crate::level::{Level, LevelStats, UpdateLevelStats};
use crate::state::LevelState;
use crate::{GRAVITY, PHYSICS_DT};

pub struct ThrowPlugin;

impl Plugin for ThrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (create_aiming_block.run_if(in_state(LevelState::Playing)),),
        )
        .add_systems(OnExit(LevelState::Playing), remove_simulation_system)
        .add_systems(
            PreUpdate,
            (simulate_throw_system,).run_if(in_state(LevelState::Playing)),
        )
        .add_systems(
            Update,
            (
                fill_throw_queue,
                update_aim_system,
                update_aim_from_mouse_position_system,
                mousewheel_aim_force_system,
                throw_system,
                update_aiming_block_position,
            )
                .run_if(in_state(LevelState::Playing)),
        )
        .init_resource::<Aim>()
        .init_resource::<ThrowQueue>();
    }
}

#[derive(Resource, Debug)]
pub struct Aim {
    pub direction: Vec2,
    pub force_factor: f32,
    pub force: f32,
    pub rotation: f32,
}
impl Default for Aim {
    fn default() -> Self {
        Self {
            direction: Vec2::from_angle(PI / 1.5),
            force_factor: 0.0,
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

pub fn remove_simulation_system(
    mut commands: Commands,
    mut query: Query<(Entity), With<TargetIndicator>>,
    mut aiming_block_query: Query<(Entity), With<Aiming>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in aiming_block_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn simulate_throw_system(
    mut commands: Commands,
    aim: Res<Aim>,
    aimed_block: Query<
        (Entity, &Collider, &Transform, &ReadMassProperties),
        (With<Block>, With<Aiming>),
    >,
    has_falling_block: Query<Entity, With<Falling>>,
    rapier_context: Res<RapierContext>,
    old_target_indicators: Query<Entity, With<TargetIndicator>>,
    magnets: Query<
        (Entity, &Transform, &MagneticEffect),
        (Without<Falling>, Without<Aiming>, With<Block>),
    >,
    other_blocks: Query<
        (Entity, &Transform),
        (
            With<Block>,
            Without<MagneticEffect>,
            Without<Aiming>,
            Without<TargetIndicator>,
        ),
    >,
    am_i_magnet: Query<&MagneticEffect>,
    mut assets: ResMut<AssetServer>,
) {
    // remove previous target indicators
    for entity in old_target_indicators.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if has_falling_block.iter().count() > 0 {
        return;
    }

    if let Ok((aimed, aimed_collider, aimed_transform, mass)) = aimed_block.get_single() {
        let mut t = 0.0;
        let dt = PHYSICS_DT;

        let shape = aimed_collider.clone();
        let mut transform = aimed_transform.clone();
        let mut velocity = aim.velocity();
        let mut acceleration = Vec2::Y * GRAVITY;

        let mut hit = false;

        let mut steps = vec![];

        while t < 2.0 {
            let angle = Vec2::Y.angle_between((transform.rotation * Vec3::Y).xy());

            let mut intersection = rapier_context.cast_shape(
                transform.translation.xy(),
                angle,
                velocity.linvel,
                &shape,
                dt,
                true,
                QueryFilter::default()
                    .exclude_collider(aimed)
                    .exclude_sensors(),
            );

            if let Some((entity, toi)) = intersection {
                transform.translation += Vec3::from((velocity.linvel * toi.toi, 0.0));
                hit = true;
                break;
            }

            let my_magnetic_effect = am_i_magnet.get(aimed).ok();

            let mut impulse =
                magnets
                    .iter()
                    .fold(Vec2::ZERO, |acc, (entity, magnet_transform, effect)| {
                        let mut impulse =
                            calculate_magnetic_impulse(magnet_transform, &transform, effect)
                                .unwrap_or(Vec2::ZERO);

                        if let Some(my_magnetic_effect) = my_magnetic_effect {
                            impulse += calculate_magnetic_impulse(
                                &transform,
                                magnet_transform,
                                my_magnetic_effect,
                            )
                            .unwrap_or(Vec2::ZERO);
                        }

                        impulse + acc
                    });

            if let Some(my_magnetic_effect) = my_magnetic_effect {
                impulse +=
                    other_blocks
                        .iter()
                        .fold(Vec2::ZERO, |acc, (entity, block_transform)| {
                            let mut impulse = calculate_magnetic_impulse(
                                &transform,
                                block_transform,
                                my_magnetic_effect,
                            )
                            .unwrap_or(Vec2::ZERO);
                            acc - impulse
                        });
            }

            velocity.linvel += impulse / mass.mass;

            velocity.linvel += acceleration * dt;
            transform.rotation = transform.rotation * Quat::from_rotation_z(velocity.angvel * dt);
            transform.translation += Vec3::from((velocity.linvel * dt, 0.0));

            steps.push(transform.clone());

            t += dt;
        }

        // println!("transform: {:?}", transform);
        // println!("velocity: {:?}", velocity);
        // println!("acceleration: {:?}", acceleration);
        // println!("hit: {:?}", hit);
        //rapier_context.intersection_with_shape()

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
    mut launch_platform_query: Query<&Transform, With<LaunchPlatform>>,
) {
    if query.iter().count() == 0 {
        if let Some(block_type) = throw_queue.queue.pop() {
            let launch_platform_transform = launch_platform_query.single();
            Block::spawn(
                &mut commands,
                block_type,
                Vec2::new(
                    launch_platform_transform.translation.x,
                    launch_platform_transform.translation.y + 35.0,
                ),
                &mut assets,
            );
        }
    }
}

pub fn update_aiming_block_position(
    mut query: Query<(Entity, &mut Transform), (With<Aiming>, Without<LaunchPlatform>)>,
    mut launch_platform_query: Query<&Transform, With<LaunchPlatform>>,
) {
    let launch_platform_transform = launch_platform_query.single();
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x = launch_platform_transform.translation.x;
        transform.translation.y = launch_platform_transform.translation.y + 35.0;
    }
}

pub fn setup_throw_queue(mut throw_queue: ResMut<ThrowQueue>, level: Res<Level>) {
    if let Some(max_blocks) = level.max_blocks {
        for _ in 0..max_blocks {
            throw_queue.queue.push(BlockType::random());
        }
        throw_queue.target_length = 0;
    } else {
        throw_queue.target_length = 3;
    }
}

pub fn fill_throw_queue(
    mut throw_queue: ResMut<ThrowQueue>,
    level: Res<Level>,
    level_stats: Res<LevelStats>,
) {
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
        aim.force_factor += 2.0;
    } else if key_code.pressed(KeyCode::Down) {
        aim.force_factor -= 2.0;
    }

    if key_code.pressed(KeyCode::Q) {
        aim.rotation += 0.0125;
    } else if key_code.pressed(KeyCode::E) {
        aim.rotation -= 0.0125;
    }
}

pub fn update_aim_from_mouse_position_system(
    mut aim: ResMut<Aim>,
    mut query: Query<&mut Transform, With<Aiming>>,
    mouse_position: Res<CursorCoords>,
    mut input: ResMut<Input<KeyCode>>,
) {
    let mouse_position = mouse_position.0;
    let mut query = query.iter_mut();
    if let Some(mut transform) = query.next() {
        // let force =
        //     calculate_velocity_for_throw(transform.translation.xy(), mouse_position, aim.direction);
        //
        // aim.force = force;

        let mut shot = None;

        let mut min_force = 0.0;

        let force_factor = aim.force_factor;

        let direct_aim = force_factor > 0.0;

        for _ in 0..1000 {
            // let switch_aim = input.pressed(KeyCode::ControlLeft);
            //
            // let angle =
            //     Vec2::Y.angle_between((transform.translation.xy() - mouse_position).normalize());
            //
            // // True when the mouse is in a 45 degree angle below the platform
            // let use_direct_aim = angle < PI / 4.0 && angle > -PI / 4.0;

            shot = calculate_shot_for_target(
                transform.translation.xy(),
                mouse_position,
                min_force,
                direct_aim,
            );
            if shot.is_none() {
                min_force += 1.0;
            } else {
                break;
            }
        }

        let force = min_force + min_force * force_factor.abs();

        shot = calculate_shot_for_target(
            transform.translation.xy(),
            mouse_position,
            force,
            direct_aim,
        );

        if let Some(shot) = shot {
            aim.force = force;
            aim.direction = shot.normalize();
        }
    }
}

pub fn mousewheel_aim_force_system(
    mut aim: ResMut<Aim>,
    mut mouse_wheel_input: EventReader<MouseWheel>,
) {
    let factor = 0.0005;
    for event in mouse_wheel_input.read() {
        match event.unit {
            MouseScrollUnit::Pixel => {
                aim.force_factor += event.y * factor;
            }
            MouseScrollUnit::Line => {
                aim.force_factor += event.y * 10.0 * factor;
            }
        }
    }
}

fn calculate_shot_for_target(
    target: Vec2,
    launch_pos: Vec2,
    velocity: f32,
    use_direct_aim: bool,
) -> Option<Vec2> {
    let xp = target.x - launch_pos.x;
    let y = target.y - launch_pos.y;
    let g = GRAVITY;
    let v = velocity;
    let angle1: f32;
    let angle2: f32;

    let tmp = v.powi(4) - g * (g * xp.powi(2) + 2.0 * y * v.powi(2));

    if tmp < 0.0 {
        return None;
    } else {
        angle1 = (v.powi(2) + tmp.sqrt()).atan2(g * xp);
        angle2 = (v.powi(2) - tmp.sqrt()).atan2(g * xp);
    }

    let angle = if use_direct_aim { angle2 } else { angle1 };

    let direction = Vec2::new(angle.cos(), angle.sin()).normalize();
    let force = Vec2::new(direction.x * v, direction.y * v);

    Some(force)
}

pub fn throw_system(
    mut commands: Commands,
    mut input: ResMut<Input<KeyCode>>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    mut touch_input: ResMut<Touches>,
    mut aim: ResMut<Aim>,
    mut query: Query<(Entity), With<Aiming>>,
    mut update_level_stats_event: EventWriter<UpdateLevelStats>,
) {
    if input.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || (touch_input.any_just_released() && touch_input.iter().count() == 0)
    {
        for entity in query.iter_mut() {
            commands
                .entity(entity)
                .remove::<Aiming>()
                .remove::<Sensor>()
                .insert((
                    Falling,
                    RigidBody::Dynamic,
                    Sleeping::disabled(),
                    aim.velocity(),
                ));

            update_level_stats_event.send(UpdateLevelStats::BlockThrown);
        }
    }
}