use bevy::prelude::*;

pub(crate) trait VectorMath {
    fn scaled_vector_to(&self, to: &Self, scale: f32) -> Self;
    fn is_in_triangle(&self, a: &Self, b: &Self, c: &Self) -> bool;
    fn rotated_by(&self, angle: f32) -> Self;
}

impl VectorMath for Vec2 {
    fn scaled_vector_to(&self, to: &Self, scale: f32) -> Self {
        let delta_x = to.x - self.x;
        let delta_y = to.y - self.y;

        Vec2::new(delta_x, delta_y).normalize_or_zero() * scale
    }

    fn is_in_triangle(&self, a: &Self, b: &Self, c: &Self) -> bool {
        let w_1: f32 = (a.x * (c.y - a.y) + (self.y - a.y) * (c.x - a.x) - self.x * (c.y - a.y))
            / ((b.y - a.y) * (c.x - a.x) - (b.x - a.x) * (c.y - a.y));
        let w_2: f32 = (self.y - a.y - w_1 * (b.y - a.y)) / (c.y - a.y);

        w_1 >= 0.0 && w_2 >= 0.0 && (w_1 + w_2) <= 1.0
    }

    fn rotated_by(&self, angle: f32) -> Self {
        let rotated_x = self.x * angle.cos() - self.y * angle.sin();
        let rotated_y = self.x * angle.sin() + self.y * angle.cos();

        Vec2::new(rotated_x, rotated_y)
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
