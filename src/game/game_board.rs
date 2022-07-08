use super::{character_display::Character, suggestion::Suggestion, GameDifficulty, Guess};
use crate::game::logic::{compare_input, create_suggestion, pick_word};
use regex::Regex;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::{
    events::{InputEvent, KeyboardEvent},
    function_component, html, use_effect_with_deps, use_node_ref, use_reducer, use_state, Callback,
    Html, Properties, Reducible,
};

#[derive(Properties, PartialEq)]
pub struct GameBoardProps {
    pub exit_handler: Callback<()>,
    pub difficulty: GameDifficulty,
}

fn get_word() -> String {
    pick_word()
}

enum GameAction {
    Submit(Guess),
    Reset,
}

#[derive(PartialEq)]
struct GameState {
    pub guesses: Vec<Guess>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            guesses: [].to_vec(),
        }
    }
}

impl Reducible for GameState {
    /// Reducer Action Type
    type Action = GameAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_guesses = match action {
            GameAction::Submit(new_feedback) => {
                let mut new_guesses = self.guesses.clone();
                new_guesses.push(new_feedback);
                new_guesses
            }
            GameAction::Reset => [].to_vec(),
        };

        Self {
            guesses: next_guesses,
        }
        .into()
    }
}

#[function_component(GameBoard)]
pub fn game_board(
    GameBoardProps {
        exit_handler,
        difficulty,
    }: &GameBoardProps,
) -> Html {
    // let difficulty = use_state(|| match game_difficulty {
    //     Easy => "Easy",
    //     Medium => "Medium",
    //     Hard => "Hard",
    // });
    let max_guesses = use_state::<i8, _>(|| match difficulty {
        GameDifficulty::Easy => 10,
        GameDifficulty::Medium => 7,
        GameDifficulty::Hard => 5,
    });
    let guess_count = use_state::<i8, _>(|| 0);
    let selected_word = use_state(|| get_word());
    let input_ref = use_node_ref();
    let current_guess = use_state(|| "".to_string());
    let input_value = (*current_guess).clone();
    let game_over = use_state(|| false);
    let victory = use_state(|| false);
    let submittable = use_state(|| false);
    let suggestion = use_state(|| "".to_string());
    let suggestion_accuracy = use_state::<f32, _>(|| 0 as f32);
    // create a stateful variable that is a vector of vectors
    let game_logic = use_reducer(GameState::default);
    let max_length = 5;

    let return_to_menu = {
        let exit_handler = exit_handler.clone();
        Callback::from(move |_| {
            exit_handler.emit(());
        })
    };

    let check_answer = {
        let current_guess = current_guess.clone();
        let selected_word = selected_word.clone();
        let victory = victory.clone();
        let game_logic = game_logic.clone();
        let game_over = game_over.clone();
        let submittable = submittable.clone();
        let guess_count = guess_count.clone();
        let max_guesses = max_guesses.clone();
        move || {
            // Method to check the current guess against the selected word to give feedback
            let feedback = compare_input(&current_guess, &selected_word);
            // Add the feedback to the guesses vector
            game_logic.dispatch(GameAction::Submit(Guess {
                word: current_guess.to_string(),
                feedback: feedback.clone(),
            }));
            let new_guess_count = *guess_count + 1;

            current_guess.set("".to_string());
            submittable.set(false);
            let mut user_won = true;
            for f in feedback {
                if !f.in_correct_location {
                    user_won = false;
                }
            }
            victory.set(user_won);
            let is_game_over = user_won || &new_guess_count > &max_guesses;
            game_over.set(is_game_over);
            guess_count.set(new_guess_count);
        }
    };

    let submit_answer = {
        let check_answer = check_answer.clone();

        Callback::from(move |_| {
            check_answer();
        })
    };

    let handle_input = {
        let current_guess = current_guess.clone();
        let input_ref = input_ref.clone();
        let submittable = submittable.clone();
        Callback::from(move |_input: InputEvent| {
            let re = Regex::new(r"^[a-z]{5}$").unwrap();
            if let Some(input_element) = input_ref.cast::<HtmlInputElement>() {
                let new_value = format!("{}", input_element.value());
                submittable.set(re.is_match(&new_value));
                current_guess.set(new_value);
            }
        })
    };

    let handle_key_down = {
        let current_guess = current_guess.clone();
        let selected_word = selected_word.clone();
        let check_answer = check_answer.clone();
        Callback::from(move |input: KeyboardEvent| {
            let key = input.key();
            if key == "Enter" && &current_guess.len() == &selected_word.len() {
                check_answer();
            }
        })
    };

    let play_again = {
        let game_logic = game_logic.clone();
        let selected_word = selected_word.clone();
        let victory = victory.clone();
        let current_guess = current_guess.clone();
        let game_over = game_over.clone();
        let submittable = submittable.clone();
        let guess_count = guess_count.clone();
        Callback::from(move |_| {
            selected_word.set(get_word());
            current_guess.set("".to_string());
            victory.set(false);
            game_over.set(false);
            submittable.set(false);
            game_logic.dispatch(GameAction::Reset);
            guess_count.set(0);
        })
    };

    // when guess count goes up, create a suggestion
    use_effect_with_deps(
        {
            let guesses = game_logic.guesses.clone();
            let suggestion = suggestion.clone();
            let suggestion_accuracy = suggestion_accuracy.clone();
            move |_| {
                let (new_suggestion, percentage) = create_suggestion(&guesses);
                suggestion.set(new_suggestion);
                suggestion_accuracy.set(percentage);
                || ()
            }
        },
        (*game_logic).guesses.clone(),
    );

    html! {
      <>
        <div class="row justify-content-center">
          <div class="col-auto">
            <h3> { "Welcome to the Word Game"}</h3>
          </div>
        </div>
        <div class="row justify-content-center">
          <div class="col-auto">
            <h5> { "Try to guess the 5 letter word."}</h5>
          </div>
        </div>
        // Main game area
        // <div class="row justify-content-center">
        //   <div class="col-auto">{ &*selected_word.to_string() }</div>
        // </div>

        {game_logic
          .guesses
          .clone()
          .into_iter()
          .map(|guess| {
              html! {
                <>
                  <div key={guess.word.clone()} class="row justify-content-center">
                    {guess.feedback.into_iter().map(|character| {
                      html! {
                        <>
                          <Character character={character.letter} feedback={character} />
                        </>
                      }
                    }).collect::<Html>()}
                  </div>
                </>
              }
          })
          .collect::<Html>()}
        // <div class="row justify-content-center">
        //   <div class="col-auto">{ &*current_guess.to_string() }</div>
        // </div>
        {if !*game_over {
          html!{<Suggestion suggestion={format!("{}", *suggestion)} accuracy={*suggestion_accuracy} />}
        } else { html!{""}}}
          <div class="row mt-2 mb-2 justify-content-center">
          {if !*game_over {
            html! {
              <div class="col-auto">
             <input
                ref={input_ref}
                type="text"
                pattern="[a-z]"
                maxlength={max_length.to_string()}
                class="form-control form-control-lg"
                oninput={handle_input}
                onkeydown={handle_key_down}
                value={input_value}
              />
            </div>}
          } else if *victory {
            html! {
              <div class="col-auto">
                <h3 class="text-success"> {"You Won!"}</h3>
              </div>
            }
          } else {
            html! {
              <div class="col-auto">
                <h3 class="text-danger"> {"You Lost!"}</h3>
              </div>
            }
          }}
        </div>



        // Footer Area
        <div class="row justify-content-center">
          <div class="col-auto">
          {if !*game_over {
            html!{

                <button disabled={!*submittable} class="btn btn-lg btn-primary" onclick={submit_answer}>{ "Submit Guess" }</button>
            }
          } else {
            html!{
              <button class="btn btn-lg btn-success" onclick={play_again}>{ "Play Again" }</button>

          }}}
          </div>
          <div class="col-auto">
            <button class="btn btn-lg btn-warning" onclick={return_to_menu}>{ "End Game" }</button>
          </div>

        </div>
      </>
    }
}
