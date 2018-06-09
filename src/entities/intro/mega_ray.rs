use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::{DrawMode, Point2};
use ggez::Context;

use entities::{Entity, EntityData};
use messages::{Message, MessageSender};
use palette::Palette;

use math::VectorUtils;

const PRECISION: f32 = 0.5;

pub struct MegaRay {
    entity_data: EntityData,
    cycle: f32,
}

impl MegaRay {
    pub fn new(p: Point2) -> MegaRay {
        MegaRay {
            entity_data: EntityData {
                pos: p,
                z_order: 10.0,
                ..EntityData::new()
            },
            cycle: 0.0,
        }
    }
}

impl Entity for MegaRay {
    fn entity_data_mut(&mut self) -> &mut EntityData {
        &mut self.entity_data
    }
    fn entity_data(&self) -> &EntityData {
        &self.entity_data
    }
    fn update(&mut self, _ctx: &mut Context) {
        self.cycle += 0.1;
    }
    fn render(&mut self, ctx: &mut Context) {
        let cycle = self.cycle;

        if cycle % 0.2 > 0.1 {
            graphics::set_color(ctx, Color::from(Palette::Light(0.4))).unwrap();
            graphics::circle(
                ctx,
                DrawMode::Fill,
                self.entity_data.pos,
                60.0 + (cycle * 23.0).sin() * 60.0,
                PRECISION
            ).unwrap();
            graphics::line(
                ctx,
                &[self.entity_data.pos, self.entity_data.pos.add(Point2::new(0.0, 400.0))],
                40.0
            ).unwrap();
            graphics::circle(
                ctx,
                DrawMode::Fill,
                self.entity_data.pos,
                40.0,
                PRECISION
            ).unwrap();
        }
        if cycle % 0.3 > 0.2 {
            graphics::circle(
                ctx,
                DrawMode::Fill,
                self.entity_data.pos,
                60.0 + (cycle * 0.6).sin() * 20.0,
                PRECISION
            ).unwrap();
            graphics::line(
                ctx,
                &[self.entity_data.pos, self.entity_data.pos.add(Point2::new(0.0, 400.0))],
                35.0 + (cycle * 9.0).sin() * 30.0
            ).unwrap();
            graphics::circle(
                ctx,
                DrawMode::Fill,
                self.entity_data.pos,
                35.0 + (cycle * 9.0).sin() * 30.0,
                PRECISION
            ).unwrap();
        }
    }
    fn receive_message(&mut self, _sender: MessageSender, message: Message) {
        match message {
            Message::Kill => {
                self.entity_data.alive = false
            },
            _ => ()
        }
    }
}
