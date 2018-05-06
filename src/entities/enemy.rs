use ggez::Context;
use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::graphics::Color;
use ggez::timer::get_delta;

use palette::Palette;
use entities::{Entity, EntityData, EntityTag};
use messages::{MessageSender, Message};

use std::time::Duration;
use bezier2::Bezier;

use mekano::Mekano;


const PRECISION : f32 = 0.5;


pub trait EnemyPath {
    fn get(&self, t: f32) -> Point2;
}

pub struct Enemy<P> where P: EnemyPath {
    path: P,
    current_duration: Duration,
    duration: Duration,
    entity_data: EntityData,
    cycle: f32,
    animation_speed: f32,
    mekano: Mekano<f32>,
}

impl<P> Enemy<P> where P: EnemyPath {
    pub fn new(path: P, duration: Duration) -> Self {
        let pos = path.get(0.0);
        Self {
            path,
            duration,
            current_duration: Duration::from_secs(0),
            entity_data: EntityData::new_at(pos),
            cycle: 0.0,
            animation_speed: 1.0,
            mekano: Self::generate_mekano_model(),
        }
    }

    fn generate_mekano_model() -> Mekano<f32> {
        Mekano::End(5.0)
    }
}

fn duration_ratio(a: Duration, b: Duration) -> f32 {
    let t_a = (a.as_secs() as f32) + (a.subsec_nanos() as f32) * 1e-9;
    let t_b = (b.as_secs() as f32) + (b.subsec_nanos() as f32) * 1e-9;
    t_a / t_b
}

impl<P> Entity for Enemy<P> where P: EnemyPath {
    fn entity_data_mut(&mut self) -> &mut EntityData { &mut self.entity_data }
    fn entity_data(&self) -> &EntityData { &self.entity_data }
    fn update(&mut self, ctx: &mut Context) {
        let delta = get_delta(ctx);
        self.current_duration += delta;
        if self.current_duration > self.duration {
            self.entity_data.alive = false;
        }
        let path_position = duration_ratio(self.current_duration, self.duration);
        self.entity_data.pos = self.path.get(path_position);
    }
    fn render(&mut self, ctx: &mut Context) {
        self.cycle += 0.1;
        graphics::set_color(ctx, Color::from(Palette::Player)).unwrap();
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.entity_data.pos,
            20.0 + (self.cycle * self.animation_speed).sin() * 2.0,
            PRECISION
        ).unwrap();
    }
    fn receive_message(&mut self, _sender: MessageSender, _message: Message) {
    }
    fn get_tag(&self) -> EntityTag { EntityTag::Enemy }
}

impl EnemyPath for Bezier {
    fn get(&self, t: f32) -> Point2 {
        self.get(t)
    }
}

