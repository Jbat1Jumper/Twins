use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;
use mekano::Mekano;

pub trait MekanoRender2D {
    type Data: MekanoRender2DData;

    fn render(mekano: &Mekano<Self::Data>, ctx: &mut Context);
}

pub trait MekanoRender2DData {
    fn origin(&self) -> Point2;
}

impl<Data> MekanoRender2D for Mekano<Data>
where
    Data: MekanoRender2DData,
{
    type Data = Data;

    fn render(mekano: &Mekano<Data>, ctx: &mut Context) {
        match mekano {
            &Mekano::End(ref d) => {

            }
            &Mekano::Segment(ref d, ref j) => {

            }
            &Mekano::Split(ref d, ref j1, ref j2) => {

            }
        }
    }
}

