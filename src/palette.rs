use ggez::graphics::Color;
use std::convert::From;
use na;

pub enum Palette {
    Black,
    Light(f32),
    Player,
}

impl From<Palette> for Color {
    fn from(p: Palette) -> Color {
        match p {
            Palette::Black => Color::from_rgb(5, 0, 10),
            Palette::Light(intensity) => {
                let i = na::clamp(intensity, 0.0, 1.0);
                let v = (i * 60.0) as u8;
                Color::from_rgb(180+v, 150+v, 195+v)
            },
            Palette::Player => Color::from_rgb(190, 200, 250)
        }
    }
}
