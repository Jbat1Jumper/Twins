use ggez::graphics::Point2;

pub trait VectorUtils {
    fn rotate(self, angle: f32) -> Self;
    fn add(self, other: Self) -> Self;
}

impl VectorUtils for Point2 {
    fn rotate(self, angle: f32) -> Point2 {
        use na::geometry::Rotation2;
        Rotation2::new(angle) * self
    }
    fn add(self, other: Point2) -> Point2 {
        Point2::new(self.x + other.x, self.y + other.y)
    }
}
