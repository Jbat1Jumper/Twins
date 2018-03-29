use entities::{EntityId, EntityTag};

#[derive(Copy, Clone)]
pub enum MessageDestination {
    Entity(EntityId),
    Tag(EntityTag),
    All,
}

pub trait IntoMessageDestination {
    fn message_destination(&self) -> MessageDestination;
}

impl IntoMessageDestination for MessageDestination {
    fn message_destination(&self) -> MessageDestination { *self }
}

pub enum MessageSender {
    Entity(EntityId),
    God,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
pub enum Message {
    Move(Direction, f32),
    Shoot,
    Start,
    Stop,
    Kill,
}
