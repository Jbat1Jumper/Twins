use ggez::event::{Keycode, Mod};
use ggez::graphics::Point2;
use math::VectorUtils;
use entities::{EntityTag, EntityTagPlayer};
use messages::{SendMessageTo, Message, Direction};
use Game;

pub struct Controller {
    p1_axis: Point2,
    p1_motion_axis: Point2,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            p1_axis: Point2::zero(),
            p1_motion_axis: Point2::zero()
        }
    }
    pub fn update(&mut self, game: &mut Game) {
        self.update_motion();
        if self.p1_motion_axis.norm() >= 0.1 {
            println!("p1_axis {:?}", self.p1_motion_axis);
            game.send_message(EntityTag::Player(EntityTagPlayer::One), Message::Move(Direction::Point(self.p1_motion_axis), 5.0));
        }
    }
    pub fn key_down_event(&mut self, _game: &mut Game, keycode: Keycode, _keymod: Mod) {
        println!("keycode {:?} down", keycode);
        let p1_axis = self.p1_axis_direction(keycode);
        self.p1_axis = self.p1_axis.add(p1_axis);
    }
    pub fn key_up_event(&mut self, _game: &mut Game, keycode: Keycode, _keymod: Mod) {
        println!("keycode {:?} up", keycode);
        let p1_axis = self.p1_axis_direction(keycode);
        self.p1_axis = self.p1_axis.sub(p1_axis);
    }
    fn p1_axis_direction(&self, keycode: Keycode) -> Point2 {
        match keycode {
            Keycode::A => Point2::left(),
            Keycode::S => Point2::down(),
            Keycode::D => Point2::right(),
            Keycode::W => Point2::up(),
            _ => Point2::zero()
        }
    }
    fn update_motion(&mut self) {
        let d = self.p1_axis.clamp(1.0);
        self.p1_motion_axis = self.p1_motion_axis.lerp(d, 0.3)
    }
}
