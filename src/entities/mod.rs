use ggez::Context;
use ggez::graphics::Point2;
use messages::{MessageSender, Message};

pub type EntityId = i32;

pub struct EntityData {
    pos: Point2,
    alive: bool,
    z_order: f32
}

impl EntityData {
    pub fn new() -> EntityData {
        EntityData {
            pos: Point2::new(0.0, 0.0),
            alive: true,
            z_order: 0.0,
        }
    }
}

pub trait Entity {
    fn entity_data_mut(&mut self) -> &mut EntityData;
    fn entity_data(&self) -> &EntityData;
    fn update(&mut self,  ctx: &mut Context);
    fn render(&mut self,  ctx: &mut Context);
    fn receive_message(&mut self, sender: MessageSender, message: Message);
    fn is_alive(&self) -> bool { self.entity_data().alive }
    fn z_order(&self) -> f32 { self.entity_data().z_order }
}


pub mod mother;
pub mod stars;
pub mod twin;
