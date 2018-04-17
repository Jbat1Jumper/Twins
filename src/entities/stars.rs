use ggez::Context;
use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::graphics::{Color, Mesh, Drawable};

use palette::Palette;
use entities::{Entity, EntityData, EntityTag};
use math::VectorUtils;
use messages::{MessageSender, Message, Direction};

use rand::{Rng, StdRng, SeedableRng};

use W_WIDTH;
use W_HEIGHT;


pub struct Stars {
    entity_data: EntityData,
    cycle: f32,
    stars: Vec<(Point2, f32)>,
    speed: Point2,
    distance: f32
}

impl Stars {
    pub fn new(distance: f32) -> Stars {
        Stars {
            entity_data: EntityData::new(),
            cycle: 0.0,
            stars: Stars::populate(distance as usize),
            distance,
            speed: Point2::new(0.0, 0.0),
        }
    }

    fn populate(seed: usize) -> Vec<(Point2, f32)> {
        let mut rng = StdRng::from_seed(&[seed, seed, seed, seed]);
        let mut v : Vec<(Point2, f32)> = Vec::new();
        for _ in 1..80 {
            let x : f32 = rng.gen();
            let y : f32 = rng.gen();
            let p = Point2::new(x * (W_WIDTH as f32), y * (W_HEIGHT as f32));
            let i : f32 = rng.gen();
            let i = 0.1 + i * 1.0;
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

        if self.speed.norm() > 0.0 {
            for &mut (ref mut point, _) in self.stars.iter_mut() {
                let npoint = point.add(self.speed.mul(self.distance/10.0));
                point.x = npoint.x % W_WIDTH as f32;
                point.y = npoint.y % W_HEIGHT as f32;
            }
        }
    }
    fn render(&mut self, ctx: &mut Context) {
        let cycle = self.cycle;

        graphics::set_color(ctx, Color::from(Palette::Light(0.0))).unwrap();
        let polygon = [
            Point2::new(0.0, -1.0),
            Point2::new(0.0, 1.0),
            Point2::new(0.0, 0.0),
            Point2::new(-1.0, 0.0),
            Point2::new(1.0, 0.0),
        ];
        let star = Mesh::new_polyline(ctx, DrawMode::Line(1.0), &polygon).unwrap();

        for pi in self.stars.iter() {
            let (p, i) = *pi;
            if cycle < i * 5.0 { continue }
            if cycle % (10.0 * i) < i { continue }
            star.draw(ctx, p, 0.0).unwrap();
        }
    }
    fn receive_message(&mut self, _sender: MessageSender, message: Message) {
        match message {
            Message::Move(Direction::Down, speed) => {
                self.speed = Point2::new(0.0, 1.0).mul(speed);
            }
            Message::Stop => {
                self.speed = Point2::new(0.0, 0.0);
            }
            _ => ()
        }
    }
    fn get_tag(&self) -> EntityTag { EntityTag::Stars }
}
