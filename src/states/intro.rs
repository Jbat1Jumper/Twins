use Game;
use entities::EntityId;
use entities::mother::{Mother, MegaRay};
use entities::stars::Stars;
use entities::twin::Twin;
use ggez::graphics::Point2;
use messages::Message;

#[derive(Debug)]
pub enum IntroState {
    Start,
    Empty(f32),
    Stars(EntityId, f32),
    Mother(EntityId, f32),
    MegaRay(EntityId, f32),
    TwinsRay(EntityId, EntityId, EntityId, f32),
    Twins(EntityId, EntityId, f32),
    MotherLeaves(f32),
    End,
}

const INTRO_SPEED : f32 = 0.6;

impl IntroState {
    pub fn update(&self, game: &mut Game) -> IntroState {
        match self {
            &IntroState::Start => IntroState::Empty(5.0 * INTRO_SPEED),
            &IntroState::Empty(time) if time > 0.0 => IntroState::Empty(time - game.delta_time()),
            &IntroState::Empty(_) => {
                let id = game.add_entity(Box::new(Stars::new()));
                IntroState::Stars(id, 5.0 * INTRO_SPEED)
            },
            &IntroState::Stars(id, time) if time > 0.0 => IntroState::Stars(id, time - game.delta_time()),
            &IntroState::Stars(_, _) => {
                let id = game.add_entity(Box::new(Mother::new(Point2::new(200.0, 200.0))));
                IntroState::Mother(id, 6.0 * INTRO_SPEED)
            },
            &IntroState::Mother(id, time) if time > 0.0 => IntroState::Mother(id, time - game.delta_time()),
            &IntroState::Mother(_, _) => {
                let id = game.add_entity(Box::new(MegaRay::new(Point2::new(200.0, 200.0))));
                IntroState::MegaRay(id, 5.0 * INTRO_SPEED)
            },
            &IntroState::MegaRay(id, time) if time > 0.0 => IntroState::MegaRay(id, time - game.delta_time()),
            &IntroState::MegaRay(ray_id, time) => {
                let id1 = game.add_entity(Box::new(Twin::new(Point2::new(200.0, 500.0))));
                let id2 = game.add_entity(Box::new(Twin::new(Point2::new(200.0, 500.0))));
                IntroState::TwinsRay(ray_id, id1, id2, 5.0 * INTRO_SPEED)
            },
            &IntroState::TwinsRay(ray_id, id1, id2, time) if time > 0.0 => IntroState::TwinsRay(ray_id, id1, id2, time - game.delta_time()),
            &IntroState::TwinsRay(ray_id, id1, id2, time) => {
                game.send_message(ray_id, Message::Kill);
                IntroState::Twins(id1, id2, 5.0 * INTRO_SPEED)
            }
            _ => IntroState::End
        }
    }
}
