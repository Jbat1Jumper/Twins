use entities::{EntityId};
use ggez::graphics::Point2;

pub enum MessageSender {
    Entity(EntityId),
    God,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Point(Point2)
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
