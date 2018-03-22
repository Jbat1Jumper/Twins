use ggez::Context;
use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::graphics::Color;

use palette::Palette;
use entities::{Entity, EntityData};
use math::VectorUtils;
use messages::{MessageSender, Message};

use rand::{Rng, StdRng, SeedableRng};
use na;

use W_WIDTH;
use W_HEIGHT;


pub struct Stars {
    entity_data: EntityData,
    cycle: f32,
    stars: Vec<(Point2, f32)>,
}

impl Stars {
    pub fn new() -> Stars {
        Stars {
            entity_data: EntityData::new(),
            cycle: 0.0,
            stars: Stars::populate(),
        }
    }

    fn populate() -> Vec<(Point2, f32)> {
        let mut rng = StdRng::from_seed(&[2, 2, 2, 2]);
        let mut v : Vec<(Point2, f32)> = Vec::new();
        for _ in 1..80 {
            let x : f32 = rng.gen();
            let y : f32 = rng.gen();
            let p = Point2::new(x * (W_WIDTH as f32), y * (W_HEIGHT as f32));
            let i : f32 = rng.gen();
            let i = 0.5 + i * 1.0;
            v.push((p, i));
        }
        v
    }
}

impl Entity for Stars {
    fn entity_data_mut(&mut self) -> &mut EntityData { &mut self.entity_data }
    fn entity_data(&self) -> &EntityData { &self.entity_data }
    fn update(&mut self, _ctx: &mut Context) {
        self.cycle += 0.1;
    }
    fn render(&mut self, ctx: &mut Context) {
        let cycle = self.cycle;

        graphics::set_color(ctx, Color::from(Palette::Light(0.0))).unwrap();

        for pi in self.stars.iter() {
            let (p, i) = *pi;
            if cycle < i * 10.0 { continue }
            if cycle % (10.0 * i) < i { continue }
            graphics::line(ctx, &[p.add(Point2::new(0.0, -1.0)), p.add(Point2::new(0.0, 1.0))], 1.0).unwrap();
            graphics::line(ctx, &[p.add(Point2::new(-1.0, 0.0)), p.add(Point2::new(1.0, 0.0))], 1.0).unwrap();
        }
    }
    fn receive_message(&mut self, sender: MessageSender, message: Message) {

    }
}
