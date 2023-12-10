use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::camera_movement::CameraMovement;
use crate::state::LevelState;
use crate::{MainCamera, FLOOR_HEIGHT, HORIZONTAL_VIEWPORT_SIZE};

pub struct CityPlugin;

impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), (setup_city))
            .add_systems(Update, update_auto_width);
    }
}

#[derive(Component)]
pub struct AutoWidth {
    aspect_ratio: f32,
    open_top: bool,
    parallax: f32,
}

pub fn setup_city(mut commands: Commands, assets: Res<AssetServer>) {
    let asset_size = Vec2::new(3840.0, 4634.0);

    let aspect_ratio = asset_size.y / asset_size.x;

    commands.spawn((
        AutoWidth {
            aspect_ratio,
            open_top: true,
            parallax: 0.5,
        },
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            texture: assets.load("parallax/05_foreground.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, aspect_ratio)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    commands.spawn((
        AutoWidth {
            aspect_ratio: 691.0 / 3840.0,
            open_top: true,
            parallax: 0.0,
        },
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -5.0),
            texture: assets.load("parallax/04_street.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 691.0 / 3840.0)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    commands.spawn((
        AutoWidth {
            aspect_ratio: 818.0 / 3840.0,
            open_top: true,
            parallax: 0.0,
        },
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            texture: assets.load("parallax/03_main.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 818.0 / 3840.0)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    commands.spawn((
        AutoWidth {
            aspect_ratio: 1465.0 / 3840.0,
            open_top: true,
            parallax: -0.2,
        },
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -20.0),
            texture: assets.load("parallax/02_middle.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1465.0 / 3840.0)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
    commands.spawn((
        AutoWidth {
            aspect_ratio: 1810.0 / 3840.0,
            open_top: true,
            parallax: -0.4,
        },
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -30.0),
            texture: assets.load("parallax/01_far_city.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1810.0 / 3840.0)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    commands.spawn((
        AutoWidth {
            aspect_ratio,
            open_top: false,
            parallax: -0.5,
        },
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -100.0),
            texture: assets.load("parallax/00_sky.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, aspect_ratio)),
                anchor: Anchor::BottomCenter,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
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
