use ggez::Context;
use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::graphics::Color;

use palette::Palette;
use entities::{Entity, EntityData};
use messages::{MessageSender, Message};

use math::VectorUtils;


const PRECISION : f32 = 0.5;


pub struct Twin {
    entity_data: EntityData,
    cycle: f32,
}

impl Entity for Twin {

    fn entity_data_mut(&mut self) -> &mut EntityData { &mut self.entity_data }
    fn entity_data(&self) -> &EntityData { &self.entity_data }
    fn update(&mut self, _ctx: &mut Context) {
        self.cycle += 0.1;
    }
    fn render(&mut self, ctx: &mut Context) {
        let cycle = self.cycle;
        graphics::set_color(ctx, Color::from(Palette::Player));
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.entity_data.pos,
            20.0,
            PRECISION
        ).unwrap();
    }
    fn receive_message(&mut self, sender: MessageSender, message: Message) {

    }
}

impl Twin {
    pub fn new(pos: Point2) -> Twin {
        Twin {
            entity_data: EntityData {
                pos,
                ..EntityData::new()
            },
            cycle: 0.0
        }
    }
}
