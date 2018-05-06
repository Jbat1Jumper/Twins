use entities::enemy::{Enemy, EnemyPath};
use ggez::graphics::Point2;
use math::VectorUtils;

use bezier2::Bezier;
use entities::{Entity, EntityTag, EntityId};
use messages::{Direction, SendMessageTo, Message};
use entities::{twin, enemy};

use std::time::Duration;

use Game;


impl EnemyPath for Bezier {
    fn get(&self, t: f32) -> Point2 {
        self.get(t)
    }
}

#[derive(Debug, Clone)]
enum State {
    Start,
    Normal
}

#[derive(Debug, Clone)]
pub struct PlayState {
    state: State,
    enemies: Vec<EntityId>,
    tenemy: f32,
}

const ENEMY_INTERVAL: f32 = 6.0;

impl PlayState {
    pub fn new() -> Self {
        Self {
            state: State::Start,
            enemies: Vec::new(),
            tenemy: ENEMY_INTERVAL,
        }
    }
    pub fn update(&self, game: &mut Game) -> Self {
        let mut new = self.clone();
        new.state = match self.state {
            State::Start => {
                game.send_message(EntityTag::Stars, Message::Move(Direction::Down, 2.0));

                game.add_entity(Box::new(twin::Twin::new(Point2::new(100.0, 500.0), twin::Player::One)));
                game.add_entity(Box::new(twin::Twin::new(Point2::new(300.0, 500.0), twin::Player::Two)));

                State::Normal
            }
            State::Normal => {
                new.tenemy -= game.delta_time();
                if new.tenemy <= 0.0 {
                    new.tenemy += ENEMY_INTERVAL;
                    new.enemies.push(
                        game.add_entity(Box::new(
                            enemy::Enemy::new(PlayState::random_path(), Duration::from_secs(3))
                        ))
                    );
                }
                State::Normal
            }
        };
        new
    }
    fn random_path() -> Bezier {
        Bezier::from(Point2::new(-100.0, -100.0), Point2::right().mul(100.0))
            .to(Point2::new(200.0, 200.0), Point2::up().mul(300.0))
            .to(Point2::new(500.0, 500.0), Point2::right().mul(100.0))
    }
}
