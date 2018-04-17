extern crate ggez;
extern crate rand;

use ggez::*;
use ggez::graphics::{Color, Point2};
use ggez::event::{Keycode, Mod};
use nalgebra as na;
use std::cmp::Ordering;
use std::time::{Instant};

mod math;
mod palette;
mod entities;
mod states;
mod messages;

use entities::{Entity, EntityId, EntityTag, EntityTagPlayer};
use palette::Palette;
use states::GameState;
use messages::{MessageSender, SendMessageTo, Message, Direction};
use math::VectorUtils;


pub const W_HEIGHT : u32 = 600;
pub const W_WIDTH : u32 = 400;


pub struct Game {
    entities: Vec<(EntityId, Box<Entity>)>,
    entity_id_counter: EntityId,
    _currently_updated_entity_id: EntityId,
}

impl Game {
    pub fn delta_time(&self) -> f32 {
        0.033
    }
    fn new_entity_id(&mut self) -> EntityId {
        self.entity_id_counter += 1;
        return self.entity_id_counter;
    }
    pub fn add_entity(&mut self, entity: Box<Entity>) -> EntityId {
        let id = self.new_entity_id();
        self.entities.push((id, entity));
        self.entities.sort_by(|ref iea, ref ieb| {
            let a = &iea.1;
            let b = &ieb.1;
            if a.z_order() > b.z_order() {
                Ordering::Greater
            }
            else {
                Ordering::Less
            }
        });
        id
    }
}

impl SendMessageTo<EntityId> for Game {
    fn send_message(&mut self, target_id: EntityId, message: Message) {
        for ie in self.entities.iter_mut() {
            let (id, ref mut entity) = *ie;
            if id == target_id {
                entity.receive_message(MessageSender::God, message);
            }
        }
    }
}

impl SendMessageTo<EntityTag> for Game {
    fn send_message(&mut self, target_tag: EntityTag, message: Message) {
        for ie in self.entities.iter_mut() {
            let (_, ref mut entity) = *ie;
            if entity.get_tag().suffices(target_tag) {
                entity.receive_message(MessageSender::God, message);
            }
        }
    }
}

pub struct Main {
    game: Game,
    current_state: GameState,
    last_time: Instant,
    profile: bool,
    debug: bool,
}

impl Main {
    fn new(_ctx: &mut Context) -> GameResult<Main> {
        let s = Main {
            game: Game {
                entities: Vec::new(),
                entity_id_counter: 0,
                _currently_updated_entity_id: 0,
            },
            current_state: GameState::Start,
            last_time: Instant::now(),
            debug: false,
            profile: false,
        };
        Ok(s)
    }
    fn init(&mut self) {
    }
}

impl event::EventHandler for Main {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        let start = Instant::now();
        let frame_time = start - self.last_time;
        if self.profile { print!("   Frame time is {}s {}ms", frame_time.as_secs(), frame_time.subsec_nanos() / 1_000_000); }
        self.last_time = start;

        for i in self.game.entities.iter_mut() {
            let (_id, ref mut entity) = *i;
            entity.update(ctx);
        }
        self.game.entities.retain(move |ie| {
            let (_id, ref entity) = *ie;
            entity.is_alive()
        });
        self.current_state = self.current_state.update(&mut self.game);

        if self.debug { println!("{:?}", self.current_state); }
        let end = Instant::now();
        let update_time = end - start;
        if self.profile { print!("   Update time is {}s {}ms", update_time.as_secs(), update_time.subsec_nanos() / 1_000_000); }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        let start = Instant::now();

        graphics::set_background_color(ctx, Color::from(Palette::Black));
        graphics::clear(ctx);

        for i in self.game.entities.iter_mut() {
            let (_id, ref mut entity) = *i;
            entity.render(ctx);
        }

        let present = Instant::now();
        let draw_time = present - start;
        if self.profile { print!("   Draw time is {}s {}ms", draw_time.as_secs(), draw_time.subsec_nanos() / 1_000_000); }

        graphics::present(ctx);

        let end = Instant::now();
        let present_time = end - present;
        if self.profile {
            print!("    Present time is {}s {}ms", present_time.as_secs(), present_time.subsec_nanos() / 1_000_000);
            println!(" ;");
        }

        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {

        let axis_speed = 1.0;

        println!("keycode {:?} {}", keycode, if repeat {"repeat"} else {""});

        let p1_axis = match keycode {
            Keycode::A => Point2::left(),
            Keycode::S => Point2::down(),
            Keycode::D => Point2::right(),
            Keycode::W => Point2::up(),
            _ => Point2::zero()
        };
        let p1_axis = p1_axis.mul(axis_speed);

        self.game.send_message(EntityTag::Player(EntityTagPlayer::One), Message::Move(Direction::Point(p1_axis), 1.0));
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title = "Twins".to_string();
    c.window_mode.width = W_WIDTH;
    c.window_mode.height = W_HEIGHT;
    let ctx = &mut Context::load_from_conf("twins", "jbat1jumper", c).unwrap();
    let state = &mut Main::new(ctx).unwrap();
    state.init();
    event::run(ctx, state).unwrap();
}
