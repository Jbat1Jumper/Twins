
pub mod graphics {
    use nalgebra::*;

    pub trait Color: Clone + Copy {
        fn into_rgba(self) -> [f32; 4];
    }

    pub enum DrawMode {
        Line(f32),
        Fill,
    }

    pub trait Graphics {
        fn clear<C: Color>(&mut self, C);
        fn present(&mut self);
        // fn draw(Drawable, position: Point2, scale: f32);
    }

    pub trait DrawPrimitives: Graphics {
        fn set_color<C: Color>(&mut self, C);
        fn circle(&mut self, DrawMode, origin: Point2<f32>, radius: f32);
        fn ellipse(&mut self, DrawMode, origin: Point2<f32>, width: f32, height: f32);
        fn line(&mut self, origin: Point2<f32>, target: Point2<f32>, width: f32);
    }

    pub trait Draw<S> {
        fn draw(&self, &mut S);
    }
}


pub mod logical {
    pub trait Update<C> {
        fn update(&mut self, &mut C);
    }
}
