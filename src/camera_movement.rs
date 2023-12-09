use crate::block::Falling;
use crate::MainCamera;
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
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mut query: Query<(
        &Transform,
        &crate::block::Block,
        Without<MainCamera>,
        Without<Falling>,
    )>,
) {
    let mut highest = 100.0;
    for (transform, block, ..) in query.iter_mut() {
        if transform.translation.y > highest {
            highest = transform.translation.y;
        }
    }

    let target_height = highest - 100.0;

    let increase = 1.0;

    if target_height > camera_movement.height {
        camera_movement.height += increase;
    } else if target_height < camera_movement.height - increase {
        camera_movement.height -= increase;
    }

    for mut transform in camera_query.iter_mut() {
        transform.translation.y = camera_movement.height;
        // camera should slowly zoom out as we get higher
        //transform.scale = Vec3::new(1.0, 1.0, 1.0) * (1.0 + camera_movement.height / 1000.0);
    }
}
