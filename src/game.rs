mod character_display;
mod game_board;
mod logic;
mod main_menu;
mod suggestion;
use crate::game::game_board::GameBoard;
use crate::game::main_menu::MainMenu;
use yew::prelude::*;
#[derive(PartialEq, Copy, Clone)]
pub enum GameDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(PartialEq, Clone)]
pub struct Feedback {
    pub letter: char,
    pub in_word: bool,
    pub in_correct_location: bool,
    pub possible_double: bool,
}

#[derive(PartialEq, Clone)]
pub struct Guess {
    word: String,
    feedback: Vec<Feedback>,
}

#[function_component(Game)]
pub fn game() -> Html {
    let selected_difficulty = use_state(|| GameDifficulty::Easy);
    let in_game = use_state(|| false);
    // Create a method to update the difficulty of the game.
    let update_difficulty = {
        let selected_difficulty = selected_difficulty.clone();
        let in_game = in_game.clone();
        // Update the difficulty of the game.
        Callback::from(move |difficulty: GameDifficulty| {
            selected_difficulty.set(difficulty);
            in_game.set(true);
        })
    };

    let end_game = {
        let in_game = in_game.clone();
        Callback::from(move |()| {
            in_game.set(false);
        })
    };

    if *in_game == false {
        html! {
          <MainMenu start_game={update_difficulty} />
        }
    } else {
        html! {
          <GameBoard exit_handler={end_game} difficulty={*selected_difficulty}/>
        }
    }
}
