use ggez::graphics::Point2;
use std::marker::Sized;

pub trait VectorUtils where Self: Sized {
    fn rotate(&self, angle: f32) -> Self;
    fn add(&self, other: Self) -> Self;
    fn mul(&self, scalar: f32) -> Self;
    fn sub(&self, other: Self) -> Self {
        self.add(other.mul(-1.0))
    }
    fn norm(&self) -> f32;
    fn unit(&self) -> Self {
        let norm = self.norm();
        self.mul(1.0/norm)
    }
}

impl VectorUtils for Point2 {
    fn rotate(&self, angle: f32) -> Point2 {
        use na::geometry::Rotation2;
        Rotation2::new(angle) * self
    }
    fn add(&self, other: Point2) -> Point2 {
        Point2::new(self.x + other.x, self.y + other.y)
    }
    fn mul(&self, scalar: f32) -> Point2 {
        Point2::new(self.x * scalar, self.y * scalar)
    }
    fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
