use entities::EntityId;


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
    Move(Direction),
    Shoot,
    Start,
    Stop,
    Kill,
}
