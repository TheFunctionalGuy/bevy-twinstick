use bevy::prelude::*;

// TODO: Consider reducing the return value to (f32, f32)
pub fn scaled_vector_between_points(from: &Vec3, to: &Vec3, scale: f32) -> Vec3 {
    let delta_x = to.x - from.x;
    let delta_y = to.y - from.y;

    Vec3::new(delta_x, delta_y, 0.0).normalize_or_zero() * scale
}

trait VectorMath {
    fn is_in_triangle(&self, a: &Self, b: &Self, c: &Self) -> bool;
}

impl VectorMath for Vec2 {
    fn is_in_triangle(&self, a: &Self, b: &Self, c: &Self) -> bool {
        let w_1: f32 = (a.x * (c.y - a.y) + (self.y - a.y) * (c.x - a.x) - self.x * (c.y - a.y))
            / ((b.y - a.y) * (c.x - a.x) - (b.x - a.x) * (c.y - a.y));
        let w_2: f32 = (self.y - a.y - w_1 * (b.y - a.y)) / (c.y - a.y);

        w_1 >= 0.0 && w_2 >= 0.0 && (w_1 + w_2) <= 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_in_triangle() {
        let point = Vec2::new(1.0, 1.0);
        let a = Vec2::new(-4.0, 2.0);
        let b = Vec2::new(6.0, 4.0);
        let c = Vec2::new(-2.0, -3.0);

        assert!(point.is_in_triangle(&a, &b, &c))
    }

    #[test]
    fn point_is_not_in_triangle() {
        let point = Vec2::new(-3.0, 6.0);
        let a = Vec2::new(-4.0, 2.0);
        let b = Vec2::new(6.0, 4.0);
        let c = Vec2::new(-2.0, -3.0);

        assert!(!point.is_in_triangle(&a, &b, &c))
    }
}
