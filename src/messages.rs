use entities::{EntityId, EntityTag};

pub enum MessageDestination {
    Entity(EntityId),
    Tag(EntityTag),
    All,
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
