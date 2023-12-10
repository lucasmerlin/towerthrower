use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResolution;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use crate::base::BasePlugin;
use crate::block::{BlockPlugin, CaughtBlock, FallingBlockCollision, SpawnTimer};
use crate::camera_movement::{camera_movement_system, CameraMovement};
use crate::cursor_system::{my_cursor_system, CursorCoords};
use crate::debris::DebrisPlugin;
use crate::effect::EffectPlugin;
use crate::environment::EnvironmentPlugin;
use crate::floor::FloorPlugin;
use crate::launch_platform::LaunchPlatformPlugin;
use crate::level::LevelPlugin;
use crate::level_intro_dialog::LevelIntroDialogPlugin;
use crate::level_ui::LevelUiPlugin;
use crate::state::StatePlugin;
use crate::target_height_indicator::TargetHeightIndicatorPlugin;
use crate::throw::ThrowPlugin;
use crate::ui::UiPlugin;

mod base;
mod block;
mod camera_movement;
mod consts;
mod cursor_system;
mod debris;
mod effect;
mod environment;
mod floor;
mod launch_platform;
mod level;
mod level_intro_dialog;
mod level_ui;
mod state;
mod target_height_indicator;
mod throw;
mod ui;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub const PIXELS_PER_METER: f32 = 1.0;
pub const GRAVITY: f32 = -9.81 * PIXELS_PER_METER * 2.0;

pub const PHYSICS_DT: f32 = 1.0 / 60.0;

pub const HORIZONTAL_VIEWPORT_SIZE: f32 = 45.0;
pub const _4K_H_RESOLUTION: f32 = 3840.0;
pub const ASSET_SCALE: f32 = HORIZONTAL_VIEWPORT_SIZE / _4K_H_RESOLUTION;
pub const FLOOR_HEIGHT: f32 = 1.0;

pub const CAR_MIN_HEIGHT: f32 = FLOOR_HEIGHT + 2.0;

pub const CAR_MAX_HEIGHT: f32 = 7.0;

pub const CAR_SCALE: f32 = 0.5;

pub const CAR_RATE: f32 = 0.75;

pub const BARREL_LENGTH: f32 = 3.5;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(250, 225, 124)))
        .add_plugins((
            // Bevy plugins
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tower Thrower".into(),
                    fit_canvas_to_parent: true,
                    // resolution: WindowResolution::new(1280.0, 720.0)
                    //     .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER)
                .in_fixed_schedule(),
            {
                let mut debug = RapierDebugRenderPlugin::default();
                debug.style.rigid_body_axes_length = 0.3;
                //debug.enabled = false;
                debug
            },
            EguiPlugin,
            WorldInspectorPlugin::new(),
            // Game plugins
            (
                BlockPlugin,
                LevelPlugin,
                StatePlugin,
                BasePlugin,
                FloorPlugin,
                DebrisPlugin,
                EnvironmentPlugin,
                EffectPlugin,
            ),
            (
                ThrowPlugin,
                LaunchPlatformPlugin,
                TargetHeightIndicatorPlugin,
                LevelUiPlugin,
                UiPlugin,
                LevelIntroDialogPlugin,
            ),
        ))
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(
            Update,
            (
                my_cursor_system,
                //block_collision,
                camera_movement_system,
                despawn_lost_entities,
            ),
        )
        // .add_systems(PostUpdate, )
        .init_resource::<CursorCoords>()
        .init_resource::<SpawnTimer>()
        .init_resource::<CameraMovement>()
        .insert_resource(AssetMetaCheck::Never)
        .add_event::<CaughtBlock>()
        .add_event::<FallingBlockCollision>()
        //.insert_resource(Time::<Fixed>::from_seconds(0.25))
        .run();
}

pub fn setup_graphics(mut commands: Commands, mut assets: ResMut<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 0.0),
            projection: OrthographicProjection {
                far: 1000.0,
                near: -1000.0,
                scaling_mode: ScalingMode::FixedHorizontal(HORIZONTAL_VIEWPORT_SIZE),
                ..OrthographicProjection::default()
            },
            ..default()
        },
        MainCamera,
    ));
}

pub fn setup_physics(
    mut commands: Commands,
    mut config: ResMut<RapierConfiguration>,
    mut context: ResMut<RapierContext>,
) {
    config.gravity = Vec2::Y * GRAVITY;
    config.timestep_mode = TimestepMode::Fixed {
        dt: PHYSICS_DT,
        substeps: 1,
    };

    //frame_pace_settings.limiter = Limiter::from_framerate(framerate as f64);

    // config.timestep_mode = TimestepMode::Interpolated {
    //     substeps: 1,
    //     dt: 1.0 / 60.0,
    //     time_scale: 1.0,
    // };

    // context.integration_parameters.max_velocity_iterations = 30;
    // context
    //     .integration_parameters
    //     .max_velocity_friction_iterations = 30;
}

pub fn despawn_lost_entities(mut commands: Commands, mut query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x < -HORIZONTAL_VIEWPORT_SIZE
            || transform.translation.x > HORIZONTAL_VIEWPORT_SIZE
            || transform.translation.y < -10.0
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}
