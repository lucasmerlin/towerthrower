use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::WindowResolution;
use bevy_egui::EguiPlugin;
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
use crate::level_ui::LevelUiPlugin;
use crate::state::StatePlugin;
use crate::target_height_indicator::TargetHeightIndicatorPlugin;
use crate::throw::ThrowPlugin;
use crate::ui::UiPlugin;

mod base;
mod block;
mod camera_movement;
mod cursor_system;
mod debris;
mod effect;
mod environment;
mod floor;
mod launch_platform;
mod level;
mod level_ui;
mod state;
mod target_height_indicator;
mod throw;
mod ui;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub const PIXELS_PER_METER: f32 = 100.0;
pub const GRAVITY: f32 = -9.81 * PIXELS_PER_METER;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(0, 148, 255)))
        .add_plugins((
            // Bevy plugins
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Tower Thrower".into(),
                    fit_canvas_to_parent: true,
                    resolution: WindowResolution::new(1280.0, 720.0)
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER)
                .in_fixed_schedule(),
            RapierDebugRenderPlugin::default(),
            EguiPlugin,
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
        .run();
}

pub fn setup_graphics(mut commands: Commands, mut assets: ResMut<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 20.0, 0.0),
            projection: OrthographicProjection {
                far: 1000.0,
                near: -1000.0,
                scaling_mode: ScalingMode::FixedVertical(700.0),
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
    let framerate = 60.0;
    config.gravity = Vec2::Y * GRAVITY;
    config.timestep_mode = TimestepMode::Fixed {
        dt: 1.0 / framerate,
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
        if transform.translation.x < -2000.0
            || transform.translation.x > 2000.0
            || transform.translation.y < -2000.0
        {
            commands.entity(entity).despawn();
        }
    }
}
