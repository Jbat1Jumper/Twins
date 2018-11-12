
pub mod graphics {

    use nalgebra::*;


    pub trait Color: Clone + Copy {
        fn into_rgba(self) -> Vector4<f32>;
    }

    pub enum DrawMode {
        Line(f32),
        Fill,
    }


    pub trait Graphics {
        fn clear<C: Color>(C);
        fn present();
        // fn draw(Drawable, position: Point2, scale: f32);
    }

    pub trait DrawPrimitives {
        fn set_color<C: Color>(C);
        fn circle(DrawMode, origin: Point2<f32>, radius: f32);
        fn ellipse(DrawMode, origin: Point2<f32>, width: f32, height: f32);
        fn line(origin: Point2<f32>, target: Point2<f32>, width: f32);
    }

}


pub mod logical {

    use std::ops::{Index, IndexMut};

    pub trait Logical<C> {
        fn update(&mut self, &mut C);
    }

    pub trait AsLogical<C> {
        fn as_logical_mut<'a>(&'a mut self) -> Option<&'a mut Logical<C>>;
    }

    pub struct LogicUpdater;

    impl LogicUpdater {
        pub fn update<C, ID>(id: ID, ctx: &mut C)
        where
            ID: Copy,
            C: IndexMut<ID>,
            C::Output: AsLogical<C> + Clone,
        {
            let mut o = ctx[id].clone();
            
            if let Some(l) = o.as_logical_mut() {
                l.update(ctx);
            }
            ctx[id] = o;
        }
    }

}
