use entities::{EntityId};

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

pub trait SendMessageTo<T> {
    fn send_message(&mut self, destination: T, message: Message);
}
