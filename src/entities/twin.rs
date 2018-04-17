
use ggez::Context;
use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::graphics::Color;

use palette::Palette;
use entities::{Entity, EntityData, EntityTag, EntityTagPlayer};
use messages::{MessageSender, Message, Direction};

use math::VectorUtils;


const PRECISION : f32 = 0.5;


pub struct Twin {
    entity_data: EntityData,
    cycle: f32,
    _going_to: Option<Point2>,
    player: Player,
    speed: f32,
}

impl Entity for Twin {
    fn entity_data_mut(&mut self) -> &mut EntityData { &mut self.entity_data }
    fn entity_data(&self) -> &EntityData { &self.entity_data }
    fn update(&mut self, _ctx: &mut Context) {
    }
    fn render(&mut self, ctx: &mut Context) {
        self.cycle += 0.1;
        graphics::set_color(ctx, Color::from(Palette::Player)).unwrap();
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.entity_data.pos,
            20.0 + (self.cycle * self.speed).sin() * 2.0,
            PRECISION
        ).unwrap();
    }
    fn receive_message(&mut self, _sender: MessageSender, message: Message) {
        match message {
            Message::Move(Direction::Point(axis), _) => {
                let pos = self.entity_data.pos.add(axis);
                self.entity_data.pos.set(pos);
            }
            _ => ()
        }
    }
    fn get_tag(&self) -> EntityTag {
        match self.player {
            Player::One => EntityTag::Player(EntityTagPlayer::One),
            Player::Two => EntityTag::Player(EntityTagPlayer::Two)
        }
    }
}

pub enum Player {
    One,
    Two
}

impl Twin {
    pub fn new(pos: Point2, player: Player) -> Self {
        Self {
            entity_data: EntityData {
                pos,
                ..EntityData::new()
            },
            cycle: 0.0,
            _going_to: Option::None,
            speed: 2.0,
            player
        }
    }
}
