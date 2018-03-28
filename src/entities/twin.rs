use ggez::Context;
use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::graphics::Color;

use palette::Palette;
use entities::{Entity, EntityData};
use messages::{MessageSender, Message, Direction};

use math::VectorUtils;


const PRECISION : f32 = 0.5;


pub struct Twin {
    entity_data: EntityData,
    cycle: f32,
    going_to: Option<Point2>,
    speed: f32,
}

impl Entity for Twin {

    fn entity_data_mut(&mut self) -> &mut EntityData { &mut self.entity_data }
    fn entity_data(&self) -> &EntityData { &self.entity_data }
    fn update(&mut self, _ctx: &mut Context) {
        self.cycle += 0.1;
        if let Some(vector) = self.going_to {
            let delta = vector.unit().mul(self.speed);
            let pos = self.get_pos();
            self.set_pos(pos.add(delta));

            self.going_to = match vector.norm() > delta.norm() {
                true => Some(vector.sub(delta)),
                false => None,
            }
        }
    }
    fn render(&mut self, ctx: &mut Context) {
        graphics::set_color(ctx, Color::from(Palette::Player)).unwrap();
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.entity_data.pos,
            20.0,
            PRECISION
        ).unwrap();
    }
    fn receive_message(&mut self, _sender: MessageSender, message: Message) {
        match message {
            Message::Move(direction, distance) => {
                self.going_to = match direction {
                    Direction::Right => Some(Point2::new(distance, 0.0)),
                    Direction::Left => Some(Point2::new(-distance, 0.0)),
                    _ => None
                };
            }
            _ => ()
        }
    }
}

impl Twin {
    pub fn new(pos: Point2) -> Twin {
        Twin {
            entity_data: EntityData {
                pos,
                ..EntityData::new()
            },
            cycle: 0.0,
            going_to: None,
            speed: 2.0,
        }
    }
}
