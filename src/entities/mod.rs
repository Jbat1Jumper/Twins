use ggez::Context;
use ggez::graphics::Point2;
use messages::{MessageSender, Message, MessageDestination, IntoMessageDestination};

pub type EntityId = i32;

impl IntoMessageDestination for EntityId {
    fn message_destination(&self) -> MessageDestination { MessageDestination::Entity(*self) }
}

#[derive(PartialEq, Copy, Clone)]
pub enum EntityTag {
    Player,
    Stars,
    Enemy,
    Untagged
}

impl IntoMessageDestination for EntityTag {
    fn message_destination(&self) -> MessageDestination { MessageDestination::Tag(*self) }
}

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
    fn die(&mut self) { self.entity_data_mut().alive = false; }
    fn z_order(&self) -> f32 { self.entity_data().z_order }
    fn get_pos(&self) -> Point2 { self.entity_data().pos }
    fn set_pos(&mut self, pos: Point2) { self.entity_data_mut().pos = pos }
    fn get_tag(&self) -> EntityTag { EntityTag::Untagged }
}


pub mod stars;
pub mod blink;
pub mod intro;
pub mod twin;
