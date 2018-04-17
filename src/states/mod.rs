pub mod intro;

use states::intro::IntroState;
use Game;


#[derive(Debug)]
pub enum GameState {
    Start,
    Intro(IntroState),
    Play,
    _Halt,
}

impl GameState {
    pub fn update(&self, game: &mut Game) -> GameState {
        match self {
            &GameState::Start => GameState::Intro(IntroState::Start),
            &GameState::Intro(ref intro_state) => {
                match intro_state.update(game) {
                    IntroState::Go => GameState::Play,
                    x => GameState::Intro(x)
                }
            },
            &GameState::Play => { GameState::Play }
            &GameState::_Halt => { GameState::_Halt }
        }
    }
}
