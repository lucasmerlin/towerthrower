use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::MainCamera;

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct CursorCoords(pub Vec2);

pub fn my_cursor_system(
    mut mycoords: ResMut<CursorCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    touches: Res<Touches>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    let cursor_position = window
        .cursor_position()
        .or(touches.iter().next().map(|touch| touch.position()));

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = cursor_position
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}
