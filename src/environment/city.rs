use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::camera_movement::CameraMovement;
use crate::environment::rain::DarkenSpriteOnRain;
use crate::level::Level;
use crate::state::LevelState;
use crate::MainCamera;

pub struct CityPlugin;

impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup_city)
            .add_systems(Update, update_auto_width);
    }
}

#[derive(Component)]
pub struct AutoWidth {
    aspect_ratio: f32,
    open_top: bool,
    parallax: f32,
}

pub fn setup_city(mut commands: Commands, assets: Res<AssetServer>, level: Res<Level>) {
    let mut spawn = |width: f32, height: f32, z: f32, parallax: f32, open_top, name| {
        let aspect_ratio = height / width;
        commands.spawn((
            AutoWidth {
                aspect_ratio,
                open_top,
                parallax,
            },
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, z),
                texture: assets.load(format!("parallax/{}.png", name)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, aspect_ratio)),
                    anchor: Anchor::BottomCenter,
                    ..Default::default()
                },
                ..Default::default()
            },
            DarkenSpriteOnRain(1.0),
        ));
    };

    spawn(3840.0, 4634.0, 100.0, 0.5, true, "05_foreground");
    spawn(3840.0, 691.0, -5.0, 0.0, true, "04_street");
    spawn(3840.0, 818.0, -10.0, 0.0, true, "03_main");
    spawn(3840.0, 1465.0, -20.0, -0.2, true, "02_middle");
    spawn(3840.0, 1810.0, -30.0, -0.4, true, "01_far_city");
    spawn(3840.0, 4634.0, -100.0, -0.5, false, "00_sky");
}

pub fn update_auto_width(
    mut query: Query<(&AutoWidth, &mut Transform)>,
    transform: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
    movement: Res<CameraMovement>,
) {
    let (camera_transform, camera) = transform.single();
    let viewport = camera.logical_viewport_size().unwrap();
    let top_left = camera
        .viewport_to_world_2d(
            camera_transform,
            Vec2::new(-viewport.x / 2.0, viewport.y / 2.0),
        )
        .unwrap();
    let bottom_right = camera
        .viewport_to_world_2d(
            camera_transform,
            Vec2::new(viewport.x / 2.0, -viewport.y / 2.0),
        )
        .unwrap();

    let world_width = bottom_right.x - top_left.x;
    let world_height = bottom_right.y - top_left.y;

    let world_aspect_ratio = world_width / world_height;

    for (auto_width, mut transform) in &mut query.iter_mut() {
        // the images should be scaled so they always fill the screen

        transform.translation.y = -movement.height * auto_width.parallax;

        if world_aspect_ratio > auto_width.aspect_ratio || auto_width.open_top {
            // the world is wider than the image
            transform.scale.x = world_width;
            transform.scale.y = world_width;
        } else {
            // the world is taller than the image
            transform.scale.x = world_height * auto_width.aspect_ratio;
            transform.scale.y = world_height * auto_width.aspect_ratio;
        }
    }
}
