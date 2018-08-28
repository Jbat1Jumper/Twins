use mursten::Data;
use nalgebra::{Point2, Vector2};

pub trait OnKeyboard {
    fn handle(&mut self, ev: KeyboardEvent) {}
}

pub enum KeyboardEvent {
    Pressed(Key, KeyModifiers),
    Released(Key, KeyModifiers),
}

pub struct KeyModifiers {}

pub enum Key {
    A,
    S,
    D,
    Q,
    W,
    E,
}

pub trait OnMouse {
    fn handle(&mut self, ev: MouseEvent) {}
}

pub enum MouseEvent {
    Pressed(MouseButton, Point2<f32>),
    Released(MouseButton, Point2<f32>),
    Movement(Vector2<f32>, Point2<f32>),
    Wheel(Vector2<f32>),
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
}

pub trait NoKeyboard {}
pub trait NoMouse {}

impl<D> OnKeyboard for D
where
    D: Data + NoKeyboard,
{
}

impl<D> OnMouse for D
where
    D: Data + NoMouse,
{
}
