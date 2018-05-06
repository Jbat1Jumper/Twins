pub mod intro;
pub mod play;

use states::intro::IntroState;
use states::play::PlayState;
use Game;


#[derive(Debug)]
pub enum GameState {
    Start,
    Intro(IntroState),
    Play(PlayState),
    _Halt,
}

impl GameState {
    pub fn update(&self, game: &mut Game) -> GameState {
        match self {
            &GameState::Start => GameState::Intro(IntroState::Start),
            &GameState::Intro(ref intro_state) => {
                match intro_state.update(game) {
                    IntroState::Go => GameState::Play(PlayState::new()),
                    x => GameState::Intro(x)
                }
            },
            &GameState::Play(ref state) => {
                GameState::Play(state.update(game))
            }
            &GameState::_Halt => { GameState::_Halt }
        }
    }
}
