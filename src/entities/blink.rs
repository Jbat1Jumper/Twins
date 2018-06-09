use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::{DrawMode, Rect};
use ggez::Context;

use entities::{Entity, EntityData};
use messages::{Message, MessageSender};
use palette::Palette;

use W_HEIGHT;
use W_WIDTH;

pub struct Blink {
    entity_data: EntityData,
    remaining_time: f32,
    total_time: f32,
}

impl Blink {
    pub fn new(time: f32) -> Self {
        if time <= 0.0 {
            panic!("Blink time can't be less or equal than zero")
        }
        Self {
            total_time: time,
            remaining_time: time,
            entity_data: EntityData {
                z_order: 100.0,
                ..EntityData::new()
            },
        }
    }
}

impl Entity for Blink {
    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }
    fn update(&mut self, _ctx: &mut Context) {
        self.remaining_time -= 0.1;
        if self.remaining_time < 0.0 {
            self.die();
        }
    }
    fn render(&mut self, ctx: &mut Context) {
        let opacity = self.remaining_time / self.total_time;
        graphics::set_color(ctx, Color::from(Palette::Blink(opacity))).unwrap();
        graphics::rectangle(ctx, DrawMode::Fill, Rect::new(0.0, 0.0, W_WIDTH as f32, W_HEIGHT as f32)).unwrap();
    }
    fn receive_message(&mut self, _sender: MessageSender, _message: Message) { }
}
