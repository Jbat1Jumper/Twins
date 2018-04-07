use ggez::Context;
use ggez::graphics::Point2;
use messages::{MessageSender, Message};

pub type EntityId = i32;

#[derive(PartialEq, Copy, Clone)]
pub enum EntityTag {
    Player(EntityTagPlayer),
    Stars,
    Enemy,
    Untagged
}

#[derive(PartialEq, Copy, Clone)]
pub enum EntityTagPlayer {
    One,
    Two,
    Both
}

impl EntityTag {
    pub fn suffices(self, other: Self) -> bool {
        match self {
            EntityTag::Player(self_tag) => {
                match other {
                    EntityTag::Player(other_tag) => {
                        match other_tag {
                            EntityTagPlayer::One => self_tag == EntityTagPlayer::One,
                            EntityTagPlayer::Two => self_tag == EntityTagPlayer::Two,
                            _ => true
                        }
                    },
                    _ => false
                }
            },
            _ => self == other
        }
    }
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
