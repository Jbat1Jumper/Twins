use ggez::graphics;
use ggez::graphics::{Font, Point2, Text};
use ggez::Context;

use entities::Renderable;

pub struct DebugText {
    text: String,
    pos: Point2,
}

impl From<(Point2, Point2)> for DebugText {
    fn from(coord: (Point2, Point2)) -> Self {
        Self {
            text: format!("[{}, {}]", coord.0.x as i32, coord.0.y as i32),
            pos: coord.1,
        }
    }
}

impl Renderable for DebugText {
    fn render(&mut self, ctx: &mut Context) {
        let font: Font = Font::default_font().unwrap();
        let t = Text::new(ctx, &self.text, &font).unwrap();
        graphics::draw(ctx, &t, self.pos, 0.0);
    }
}
