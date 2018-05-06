use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;
use mekano::{Mekano, BxBody, BxJoint, Body, Joint};

pub trait MekanoRender2D {
    type R: RootRender2DData;
    type B: BodyRender2DData;
    type J: JointRender2DData;

    fn render(mekano: Mekano<Self::R, Self::B, Self::J>, ctx: &mut Context);
}

pub trait RootRender2DData {
    fn origin(&self) -> Point2;
}

pub enum BodyRender2DShape {
}

pub trait BodyRender2DData {
}

pub trait JointRender2DData {
}


impl<R, B, J> MekanoRender2D for Mekano<R, B, J>
where
    R: RootRender2DData,
    B: BodyRender2DData,
    J: JointRender2DData
{
    type R = R;
    type B = B;
    type J = J;

    fn render(mekano: Mekano<R, B, J>, ctx: &mut Context) {
        render_body(mekano.root, ctx)
    }
}

fn render_body<B, J>(body: BxBody<B, J>, ctx: &mut Context)
where 
    B: BodyRender2DData,
    J: JointRender2DData
{

    match *body {
        Body::End(ref d) => {

        }
        Body::Segment(ref d, ref j) => {

        }
        Body::Split(ref d, ref j1, ref j2) => {

        }
    }
}

