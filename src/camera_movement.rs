use crate::block::{Block, Falling};
use crate::launch_platform::LaunchPlatform;
use crate::{MainCamera, HORIZONTAL_VIEWPORT_SIZE};
use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraMovement {
    pub height: f32,
}

impl Default for CameraMovement {
    fn default() -> Self {
        Self { height: 0.0 }
    }
}

// Should slowly move the camera up to the highest non-falling block
// Camera is only moving up, not down
pub fn camera_movement_system(
    mut camera_movement: ResMut<CameraMovement>,
    mut camera_query: Query<(&mut Transform, &GlobalTransform, &Camera), With<MainCamera>>,
    mut query: Query<
        &Transform,
        (
            Without<MainCamera>,
            Without<Falling>,
            Or<(With<Block>, With<LaunchPlatform>)>,
        ),
    >,
) {
    let start_move_at = 20.0;

    let mut highest = start_move_at;
    for transform in query.iter_mut() {
        if transform.translation.y > highest {
            highest = transform.translation.y;
        }
    }

    let target_height = highest - start_move_at;

    let increase = 0.1;

    if target_height > camera_movement.height {
        camera_movement.height += increase;
    } else if target_height < camera_movement.height - increase {
        camera_movement.height -= increase;
    }

    for (mut transform, global_transform, camera) in camera_query.iter_mut() {
        let viewport = camera.logical_viewport_size().unwrap();

        let scene_height = HORIZONTAL_VIEWPORT_SIZE * viewport.y / viewport.x;

        transform.translation.y = camera_movement.height + scene_height / 2.0;
        // camera should slowly zoom out as we get higher
        //transform.scale = Vec3::new(1.0, 1.0, 1.0) * (1.0 + camera_movement.height / 1000.0);
    }
}
