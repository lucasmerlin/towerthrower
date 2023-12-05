mod base;
mod block;
mod block_spawner;
mod camera_movement;
mod cursor_system;
mod effect;
mod launch_platform;
mod throw;

use crate::base::{base_position, setup_base};
use crate::block::{
    block_stable_system, despawn_droped_blocks, despawn_target_beam, falling_block_collision,
    rotate_aimed_blocks, CaughtBlock, FallingBlockCollision, SpawnTimer,
};
use crate::camera_movement::{camera_movement_system, CameraMovement};
use crate::cursor_system::{my_cursor_system, CursorCoords};
use crate::effect::EffectPlugin;
use crate::launch_platform::{LaunchPlatform, LaunchPlatformPlugin};
use crate::throw::ThrowPlugin;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_rapier2d::prelude::*;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0),
            RapierDebugRenderPlugin::default(),
            EffectPlugin,
            ThrowPlugin,
            LaunchPlatformPlugin,
        ))
        .add_systems(Startup, (setup_graphics, setup_physics, setup_base))
        .add_systems(
            Update,
            (
                my_cursor_system,
                //base_position,
                //block_collision,
                rotate_aimed_blocks,
                despawn_droped_blocks,
                camera_movement_system,
                despawn_target_beam,
                falling_block_collision,
                block_stable_system,
            ),
        )
        // .add_systems(PostUpdate, )
        .init_resource::<CursorCoords>()
        .init_resource::<SpawnTimer>()
        .init_resource::<CameraMovement>()
        .add_event::<CaughtBlock>()
        .add_event::<FallingBlockCollision>()
        .run();
}

pub fn setup_graphics(mut commands: Commands, mut assets: ResMut<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 0.0),
            projection: OrthographicProjection {
                far: 1000.0,
                near: -1000.0,
                scaling_mode: ScalingMode::FixedVertical(1000.0),
                ..OrthographicProjection::default()
            },
            ..default()
        },
        MainCamera,
    ));
}

pub fn setup_physics(mut commands: Commands, mut gravity_scale: ResMut<RapierConfiguration>) {
    gravity_scale.gravity = Vec2::Y * -9.81 * 100.0;

    // /*
    //  * Ground
    //  */
    // let ground_size = 500.0;
    // let ground_height = 10.0;
    //
    // commands.spawn((
    //     TransformBundle::from(Transform::from_xyz(0.0, 10.0 * -ground_height, 0.0)),
    //     Collider::cuboid(ground_size, ground_height),
    // ));

    // /*
    //  * Create the cubes
    //  */
    // let num = 8;
    // let rad = 10.0;
    //
    // let shift = rad * 1.5 + rad;
    // let centerx = shift * (num / 2) as f32;
    // let centery = shift / 2.0;
    //
    // let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
    //
    // for j in 0usize..2000 {
    //     for i in 0..num {
    //         let x = i as f32 * shift - centerx + offset;
    //         let y = j as f32 * shift + centery + 30.0;
    //
    //         commands.spawn((
    //             TransformBundle::from(Transform::from_xyz(x, y, 0.0)),
    //             RigidBody::Dynamic,
    //             Collider::cuboid(rad, rad),
    //         ));
    //     }
    //
    //     offset -= 0.05 * rad * (num as f32 - 1.0);
    // }
}
