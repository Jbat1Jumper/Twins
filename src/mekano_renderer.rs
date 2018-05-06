use ggez::Context;
use ggez::graphics;
use ggez::graphics::{Point2, DrawMode};
use mekano::Mekano;
use math::VectorUtils;

pub trait Render {
    type Data: Data;

    fn render(&self, ctx: &mut Context);
}

pub trait Data {
    fn origin(&self) -> Point2 {
        Point2::zero()
    }
    fn rotation(&self) -> f32 {
        0.0
    }
    fn shape(&self) -> Shape {
        Shape::None
    }
}

pub enum Shape {
    Circle(f32),
    Ellipse(f32, f32),
    Sqare(f32),
    Rectangle(f32, f32),
    Rombus(f32, f32),
    None,
}

impl<D> Render for Mekano<D>
where
    D: Data,
{
    type Data = D;

    fn render(&self, ctx: &mut Context) {

        {
            const tolerance: f32 = 5.0;
            let data = self.data();
            let shape = data.shape();
            let origin = data.origin();

            match shape {
                Shape::Circle(radius) => {
                    graphics::circle(ctx, DrawMode::Fill, origin, radius, tolerance).unwrap();
                }
                _ => {}
            }
        }
        match self {
            &Mekano::End(ref d) => {

            }
            &Mekano::Segment(ref d, ref j) => {

            }
            &Mekano::Split(ref d, ref j1, ref j2) => {

            }
        }
    }
}

