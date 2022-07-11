use bevy::prelude::*;

// TODO: Consider reducing the return value to (f32, f32)
pub fn scaled_vector_between_points(from: &Vec3, to: &Vec3, scale: f32) -> Vec3 {
    let delta_x = to.x - from.x;
    let delta_y = to.y - from.y;

    Vec3::new(delta_x, delta_y, 0.0).normalize_or_zero() * scale
}
