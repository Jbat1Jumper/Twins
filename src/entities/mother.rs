use ggez::Context;
use ggez::graphics::{Point2, DrawMode};
use ggez::graphics;
use ggez::graphics::Color;

use palette::Palette;
use entities::{Entity, EntityData};
use messages::{MessageSender, Message};

use math::VectorUtils;


const PRECISION : f32 = 0.5;


pub struct Mother {
    entity_data: EntityData,
    cycle: f32,
}

impl Mother {
    pub fn new(pos: Point2) -> Mother {
        Mother {
            entity_data: EntityData {
                pos,
                ..EntityData::new()
            },
            cycle: 0.0
        }
    }

    fn render_ray(&mut self, ctx: &mut Context, lenght: f32, speed: f32, offset: f32) {
        graphics::line(
            ctx,
            &[self.entity_data.pos, self.entity_data.pos.add(Point2::new(lenght, 0.0).rotate(self.cycle * speed + offset))],
            1.0
        ).unwrap();
    }

    fn render_rays(&mut self, ctx: &mut Context, cycle: f32) {
        graphics::set_color(ctx, Color::from(Palette::Light((cycle * 1.4 + 1.0).sin()))).unwrap();
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 0.0).sin() * 10.0, -0.21, 0.0);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 1.0).sin() * 10.0, 0.25, 2.8);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 2.0).sin() * 10.0, 0.22, 2.0);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 3.0).sin() * 10.0, 0.2, 1.0);

        graphics::set_color(ctx, Color::from(Palette::Light((cycle * 2.0 + 1.0).sin()))).unwrap();
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 0.0).sin() * 10.0, -0.2, 0.0);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 1.2).sin() * 10.0, 0.26, 4.0);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 2.8).sin() * 10.0, 0.23, 5.0);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 3.5).sin() * 10.0, -0.23, 6.0);

        graphics::set_color(ctx, Color::from(Palette::Light((cycle * 2.6 + 1.0).sin()))).unwrap();
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 0.0).sin() * 10.0, -0.3, 0.5);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 1.0).sin() * 10.0, 0.25, 2.4);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 2.0).sin() * 10.0, -0.23, 2.0);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 3.0).sin() * 10.0, -0.2, 1.5);

        graphics::set_color(ctx, Color::from(Palette::Light((cycle * 4.2 + 1.0).sin()))).unwrap();
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 0.0).sin() * 10.0, -0.2, 0.2);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 1.2).sin() * 10.0, 0.26, 4.5);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 2.8).sin() * 10.0, 0.3, 5.5);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 3.5).sin() * 10.0, -0.25, 6.5);

        graphics::set_color(ctx, Color::from(Palette::Light((cycle * 5.6 + 1.0).sin()))).unwrap();
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 0.0).sin() * 10.0, -0.3, 0.5);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 1.0).sin() * 10.0, 0.25, 2.4);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 2.0).sin() * 10.0, -0.23, 2.0);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 3.0).sin() * 10.0, -0.2, 1.5);

        graphics::set_color(ctx, Color::from(Palette::Light((cycle * 1.2 + 0.4).sin()))).unwrap();
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 0.0).sin() * 10.0, 0.31, 0.2);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 1.2).sin() * 10.0, -0.32, 4.5);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 2.8).sin() * 10.0, -0.29, 5.5);
        self.render_ray(ctx, 125.0 + (cycle * 0.6 + 3.5).sin() * 10.0, 0.35, 6.5);
    }

    fn render_orbit(&mut self, ctx: &mut Context, radius: f32) {
        graphics::circle(
            ctx,
            DrawMode::Line(1.0),
            self.entity_data.pos,
            radius,
            PRECISION,
        ).unwrap();
    }

    fn render_moon_ring(&mut self, ctx: &mut Context, radius: f32, speed: f32, offset: f32, size: f32) {
        graphics::circle(
            ctx,
            DrawMode::Line(1.0),
            self.entity_data.pos.add(Point2::new(radius, 0.0).rotate(self.cycle * speed + offset)),
            size,
            PRECISION,
        ).unwrap();
    }

    fn render_orbits(&mut self, ctx: &mut Context, cycle: f32) {
        graphics::set_color(ctx, Color::from(Palette::Light(cycle.sin()))).unwrap();
        self.render_orbit(ctx, 110.0 + (cycle * 0.3 - 1.0).sin() * 7.0);
        self.render_orbit(ctx, 110.0 + (cycle * 0.3 - 0.0).sin() * 7.0);
        self.render_orbit(ctx, 110.0 + (cycle * 0.3 + 1.0).sin() * 7.0);
        self.render_orbit(ctx, 110.0 + (cycle * 0.3 + 2.0).sin() * 7.0);
        self.render_moon_ring(ctx, 110.0 + (cycle * 0.3).sin() * 6.0, 0.1, 0.0, 20.0 + (cycle * 0.9 + 1.0).sin() * 3.0);
        self.render_moon_ring(ctx, 110.0 + (cycle * 0.3).sin() * 6.0, 0.1, 0.0, 20.0 + (cycle * 0.9 + 2.0).sin() * 3.0);
        self.render_moon_ring(ctx, 110.0 + (cycle * 0.3).sin() * 6.0, 0.1, 0.0, 20.0 + (cycle * 0.9 + 3.0).sin() * 3.0);
        self.render_moon_ring(ctx, 110.0 + (cycle * 0.3).sin() * 6.0, 0.1, 0.0, 20.0 + (cycle * 0.9 + 4.0).sin() * 3.0);

        graphics::set_color(ctx, Color::from(Palette::Light((cycle + 1.0).sin()))).unwrap();
        self.render_orbit(ctx, 80.0 + (cycle * 0.3 + 0.0).sin() * 7.0);
        self.render_orbit(ctx, 80.0 + (cycle * 0.3 + 1.0).sin() * 7.0);
        self.render_orbit(ctx, 80.0 + (cycle * 0.3 + 2.0).sin() * 7.0);
        self.render_orbit(ctx, 80.0 + (cycle * 0.3 + 3.0).sin() * 7.0);
        self.render_moon_ring(ctx, 80.0 + (cycle * 0.3 + 1.0).sin() * 6.0, -0.11, 0.0, 20.0 + (cycle * 0.9 + 1.0).sin() * 3.0);
        self.render_moon_ring(ctx, 80.0 + (cycle * 0.3 + 1.0).sin() * 6.0, -0.11, 0.0, 20.0 + (cycle * 0.9 + 2.0).sin() * 3.0);
        self.render_moon_ring(ctx, 80.0 + (cycle * 0.3 + 1.0).sin() * 6.0, -0.11, 0.0, 20.0 + (cycle * 0.9 + 3.0).sin() * 3.0);
        self.render_moon_ring(ctx, 80.0 + (cycle * 0.3 + 1.0).sin() * 6.0, -0.11, 0.0, 20.0 + (cycle * 0.9 + 4.0).sin() * 3.0);

        graphics::set_color(ctx, Color::from(Palette::Light((cycle + 2.0).sin()))).unwrap();
        self.render_orbit(ctx, 50.0 + (cycle * 0.3 + 1.0).sin() * 7.0);
        self.render_orbit(ctx, 50.0 + (cycle * 0.3 + 2.0).sin() * 7.0);
        self.render_orbit(ctx, 50.0 + (cycle * 0.3 + 3.0).sin() * 7.0);
        self.render_orbit(ctx, 50.0 + (cycle * 0.3 + 4.0).sin() * 7.0);
        self.render_moon_ring(ctx, 50.0 + (cycle * 0.3 + 1.0).sin() * 6.0, 0.06, 0.0, 20.0 + (cycle * 0.9 + 1.0).sin() * 3.0);
        self.render_moon_ring(ctx, 50.0 + (cycle * 0.3 + 1.0).sin() * 6.0, 0.06, 0.0, 20.0 + (cycle * 0.9 + 2.0).sin() * 3.0);
        self.render_moon_ring(ctx, 50.0 + (cycle * 0.3 + 1.0).sin() * 6.0, 0.06, 0.0, 20.0 + (cycle * 0.9 + 3.0).sin() * 3.0);
        self.render_moon_ring(ctx, 50.0 + (cycle * 0.3 + 1.0).sin() * 6.0, 0.06, 0.0, 20.0 + (cycle * 0.9 + 4.0).sin() * 3.0);
    }

    fn render_eye(&mut self, ctx: &mut Context, cycle: f32) {
        graphics::set_color(ctx, Color::from(Palette::Light(0.0))).unwrap();
        self.render_orbit(ctx, 14.0 + (cycle * 0.3 - 2.0).sin() * 1.0);
        self.render_orbit(ctx, 14.0 + (cycle * 0.3).sin() * 1.0);
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.entity_data.pos,
            12.0,
            PRECISION,
        ).unwrap();

        graphics::set_color(ctx, Color::from(Palette::Black)).unwrap();
        graphics::ellipse(
            ctx,
            DrawMode::Fill,
            self.entity_data.pos,
            5.0 + (cycle * 0.05).sin() * 3.0,
            12.0,
            PRECISION,
        ).unwrap();

        graphics::set_color(ctx, Color::from(Palette::Light(0.0))).unwrap();
        graphics::circle(
            ctx,
            DrawMode::Fill,
            self.entity_data.pos,
            5.0 + (cycle * 0.05).sin() * 3.0,
            PRECISION,
        ).unwrap();
    }
}

impl Entity for Mother {

    fn entity_data_mut(&mut self) -> &mut EntityData { &mut self.entity_data }
    fn entity_data(&self) -> &EntityData { &self.entity_data }
    fn update(&mut self, _ctx: &mut Context) {
        self.cycle += 0.1;
    }
    fn render(&mut self, ctx: &mut Context) {
        let cycle = self.cycle;
        self.render_eye(ctx, cycle);
        self.render_orbits(ctx, cycle);
        self.render_rays(ctx, cycle);
    }
    fn receive_message(&mut self, sender: MessageSender, message: Message) {

    }
}

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
                .. EntityData::new()
            },
            cycle: 0.0,
        }
    }
}

impl Entity for MegaRay {

    fn entity_data_mut(&mut self) -> &mut EntityData { &mut self.entity_data }
    fn entity_data(&self) -> &EntityData { &self.entity_data }
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
    fn receive_message(&mut self, sender: MessageSender, message: Message) {
        match message {
            Message::Kill => {
                self.entity_data.alive = false
            },
            _ => ()
        }
    }
}
