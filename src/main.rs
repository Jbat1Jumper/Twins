extern crate ggez;
extern crate rand;

use ggez::*;
use ggez::graphics::Color;
use nalgebra as na;
use std::cmp::Ordering;

mod math;
mod palette;
mod entities;
mod states;
mod messages;

use entities::Entity;
use entities::EntityId;
use palette::Palette;
use states::GameState;
use messages::{MessageSender, Message};


pub const W_HEIGHT : u32 = 600;
pub const W_WIDTH : u32 = 400;


pub struct Game {
    entities: Vec<(EntityId, Box<Entity>)>,
    entity_id_counter: EntityId,
    currently_updated_entity_id: EntityId,
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
    pub fn send_message(&mut self, target_id: EntityId, message: Message) {
        for ie in self.entities.iter_mut() {
            let (id, ref mut entity) = *ie;
            if id == target_id {
                entity.receive_message(MessageSender::God, message);
            }
        }
    }
}

pub struct Main {
    game: Game,
    current_state: GameState,
}

impl Main {
    fn new(_ctx: &mut Context) -> GameResult<Main> {
        let s = Main {
            game: Game {
                entities: Vec::new(),
                entity_id_counter: 0,
                currently_updated_entity_id: 0,
            },
            current_state: GameState::Start
        };
        Ok(s)
    }
    fn init(&mut self) {
    }
}

impl event::EventHandler for Main {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for i in self.game.entities.iter_mut() {
            let (id, ref mut entity) = *i;
            entity.update(ctx);
        }
        self.game.entities.retain(move |ie| {
            let (_id, ref entity) = *ie;
            entity.is_alive()
        });
        self.current_state = self.current_state.update(&mut self.game);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, Color::from(Palette::Black));
        graphics::clear(ctx);

        for i in self.game.entities.iter_mut() {
            let (_id, ref mut entity) = *i;
            entity.render(ctx);
        }

        graphics::present(ctx);
        Ok(())
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
