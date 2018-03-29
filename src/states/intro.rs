use Game;
use entities::{EntityId, EntityTag};
use entities::intro::{MotherIntro, MegaRay, TwinIntro};
use entities::stars::Stars;
use entities::blink::Blink;
use entities::twin::Twin;
use ggez::graphics::Point2;
use messages::{Message, Direction, SendMessageTo};


const INTRO_SPEED : f32 = 5.0;


#[derive(Debug)]
pub struct IntroData {
    time_left: f32,
    mother: Option<EntityId>,
    ray: Option<EntityId>,
    twin1: Option<EntityId>,
    twin2: Option<EntityId>
}

impl IntroData {
    fn new() -> IntroData {
        IntroData {
            time_left: 0.0,
            mother: None,
            ray: None,
            twin1: None,
            twin2: None,
        }
    }
    fn wait(&self, time: f32) -> IntroData {
        IntroData { time_left: time, ..*self }
    }
    fn elapsed(&self, time: f32) -> IntroData {
        IntroData { time_left: self.time_left - time * INTRO_SPEED, ..*self }
    }
    fn waiting(&self) -> bool { self.time_left > 0.0 }
}

#[derive(Debug)]
pub enum IntroState {
    Start,
    Empty(IntroData),
    Stars(IntroData),
    Mother(IntroData),
    MegaRay(IntroData),
    TwinsRay(IntroData),
    Twins(IntroData),
    TwinsMoving(IntroData),
    MotherLeaves(IntroData),
    Ready(IntroData),
    Set(IntroData),
    Go,
}

impl IntroState {
    pub fn update(&self, game: &mut Game) -> IntroState {
        match self {
            &IntroState::Start => IntroState::Empty(IntroData::new().wait(20.0)),
            &IntroState::Empty(ref d) if d.waiting() => IntroState::Empty(d.elapsed(game.delta_time())),
            &IntroState::Empty(ref d) => {
                game.add_entity(Box::new(Stars::new(10.0)));
                game.add_entity(Box::new(Stars::new(20.0)));
                IntroState::Stars(d.wait(5.0))
            },
            &IntroState::Stars(ref d) if d.waiting() => IntroState::Stars(d.elapsed(game.delta_time())),
            &IntroState::Stars(ref d) => {
                let d = IntroData {
                    mother: Some(game.add_entity(Box::new(MotherIntro::new(Point2::new(200.0, 200.0))))),
                    ..*d
                };
                IntroState::Mother(d.wait(5.0))
            },
            &IntroState::Mother(ref d) if d.waiting() => IntroState::Mother(d.elapsed(game.delta_time())),
            &IntroState::Mother(ref d) => {
                let d = IntroData {
                    ray: Some(game.add_entity(Box::new(MegaRay::new(Point2::new(200.0, 200.0))))),
                    ..*d
                };
                IntroState::MegaRay(d.wait(5.0))
            },
            &IntroState::MegaRay(ref d) if d.waiting() => IntroState::MegaRay(d.elapsed(game.delta_time())),
            &IntroState::MegaRay(ref d) => {
                let d = IntroData {
                    twin1: Some(game.add_entity(Box::new(TwinIntro::new(Point2::new(200.0, 500.0))))),
                    twin2: Some(game.add_entity(Box::new(TwinIntro::new(Point2::new(200.0, 500.0))))),
                    ..*d
                };
                IntroState::TwinsRay(d.wait(5.0))
            },
            &IntroState::TwinsRay(ref d) if d.waiting() => IntroState::TwinsRay(d.elapsed(game.delta_time())),
            &IntroState::TwinsRay(ref d) => {
                if let Some(id) = d.ray {
                    game.send_message(id, Message::Kill);
                };
                let d = IntroData { ray: None, ..*d };
                IntroState::Twins(d.wait(5.0))
            },
            &IntroState::Twins(ref d) if d.waiting() => IntroState::Twins(d.elapsed(game.delta_time())),
            &IntroState::Twins(ref d) => {
                if let Some(id) = d.twin1 {
                    game.send_message(id, Message::Move(Direction::Left, 100.0));
                }
                if let Some(id) = d.twin2 {
                    game.send_message(id, Message::Move(Direction::Right, 100.0));
                }
                IntroState::TwinsMoving(d.wait(5.0))
            },
            &IntroState::TwinsMoving(ref d) if d.waiting() => IntroState::TwinsMoving(d.elapsed(game.delta_time())),
            &IntroState::TwinsMoving(ref d) => {
                if let Some(id) = d.mother {
                    game.send_message(id, Message::Move(Direction::Up, 0.05 * INTRO_SPEED));
                }
                IntroState::MotherLeaves(d.wait(8.0))
            },
            &IntroState::MotherLeaves(ref d) if d.waiting() => IntroState::MotherLeaves(d.elapsed(game.delta_time())),
            &IntroState::MotherLeaves(ref d) => {
                if let Some(id) = d.mother {
                    game.send_message(id, Message::Kill);
                }
                let d = IntroData { mother: None, ..*d };
                game.add_entity(Box::new(Blink::new(0.5)));
                IntroState::Ready(d.wait(2.0))
            }
            &IntroState::Ready(ref d) if d.waiting() => IntroState::Ready(d.elapsed(game.delta_time())),
            &IntroState::Ready(ref d) => {
                game.add_entity(Box::new(Blink::new(0.5)));
                IntroState::Set(d.wait(2.0))
            }
            &IntroState::Set(ref d) if d.waiting() => IntroState::Set(d.elapsed(game.delta_time())),
            &IntroState::Set(ref d) => {
                game.add_entity(Box::new(Blink::new(2.0)));
                if let Some(id) = d.twin1 { game.send_message(id, Message::Kill) }
                if let Some(id) = d.twin2 { game.send_message(id, Message::Kill) }

                game.send_message(EntityTag::Stars, Message::Move(Direction::Down, 2.0));

                game.add_entity(Box::new(Twin::new(Point2::new(100.0, 500.0))));
                game.add_entity(Box::new(Twin::new(Point2::new(300.0, 500.0))));

                IntroState::Go
            }
            _ => IntroState::Go
        }
    }
}
