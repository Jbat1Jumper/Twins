use ggez::graphics::Point2;
use std::marker::Sized;
use rand::Rng;
use rand::distributions::{Sample, Normal};
use std::ops::{
    Add,
    Mul,
    Sub,
};

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
    fn clamp(&self, amount: f32) -> Self;
    fn set(&mut self, other: Self);

    fn lerp(&self, other: Self, amount: f32) -> Self {
        let d = self.sub(other);
        let amount = amount.min(1.0).max(0.0);
        self.sub(d.mul(amount))
    }

    fn left() -> Self;
    fn down() -> Self;
    fn right() -> Self;
    fn up() -> Self;
    fn zero() -> Self;
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
    fn clamp(&self, amount: f32) -> Self {
        let norm = self.norm();
        if norm <= 0.001 { return Point2::zero() }
        let res = match norm < 1.0 { true => norm,
                                     false => 1.0 };
        <Self as VectorUtils>::mul(&self, res / norm)
    }
    fn set(&mut self, other: Point2) {
        self.x = other.x;
        self.y = other.y;
    }

    fn left() -> Self {
        Point2::new(-1.0, 0.0)
    }
    fn down() -> Self {
        Point2::new(0.0, 1.0)
    }
    fn right() -> Self {
        Point2::new(1.0, 0.0)
    }
    fn up() -> Self {
        Point2::new(0.0, -1.0)
    }
    fn zero() -> Self {
        Point2::new(0.0, 0.0)
    }
}

pub trait Wavize where Self: Sized + Add<Self> + Mul<Self> + Sub<Self> {
    fn wave(&self, amount: f32, tick: f32, phase: f32) -> Self;
}

impl Wavize for f32 {
    fn wave(&self, amount: f32, tick: f32, phase: f32) -> Self {
        self + (tick + phase).sin() * amount
    }
}

pub trait Randomize<RNG> where Self: Sized + Add<Self> + Mul<Self> + Sub<Self>, RNG: Rng {
    fn rand(&self, rng: &mut RNG, amount: f32) -> Self;
    fn gauss(&self, rng: &mut RNG, deviation: f32) -> Self;
}

impl<RNG> Randomize<RNG> for f32 where RNG: Rng {
    fn rand(&self, rng: &mut RNG, amount: f32) -> Self {
        let rand: f32 = rng.gen();
        let delta: f32 = rand * 2.0 - 1.0;
        self + delta * amount
    }
    fn gauss(&self, rng: &mut RNG, amount: f32) -> Self {
        let mut dist = Normal::new(*self as f64, amount as f64);
        dist.sample(rng) as f32
    }
}



#[test]
fn lerp_points() {
    let a = Point2::new(0.0, 0.0);
    let b = Point2::new(1.0, 0.0);
    let c = Point2::new(2.0, 2.0);
    assert_eq!(a.lerp(b, 0.0), a);
    assert_eq!(a.lerp(b, 0.5), Point2::new(0.5, 0.0));
    assert_eq!(a.lerp(c, 1.0), c);
    assert_eq!(c.lerp(a, 0.5), Point2::new(1.0, 1.0));
}
