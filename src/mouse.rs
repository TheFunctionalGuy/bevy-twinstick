use bevy::{prelude::*, render::camera::RenderTarget};

use crate::{components::MainCamera, weapons::shoot_weapon};

// Resources
#[derive(Default, Deref, DerefMut)]
pub struct MousePosition(Vec2);

// Plugin
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_cursor_position.before(shoot_weapon))
            .insert_resource(MousePosition::default());
    }
}

// Slightly modified version of: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn update_cursor_position(
    windows: Res<Windows>,
    mut mouse_position: ResMut<MousePosition>,
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = main_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        mouse_position.x = world_pos.x;
        mouse_position.y = world_pos.y;
    }
}
