use yew::prelude::*;

use super::GameDifficulty;

#[derive(Clone, Properties, PartialEq)]
pub struct MainMenuProps {
    pub start_game: Callback<GameDifficulty>,
}

#[function_component(MainMenu)]
pub fn main_menu(MainMenuProps { start_game }: &MainMenuProps) -> Html {
    let start_easy_game = {
        let start_game = start_game.clone();
        Callback::from(move |_| {
            start_game.emit(GameDifficulty::Easy);
        })
    };

    let start_medium_game = {
        let start_game = start_game.clone();
        Callback::from(move |_| {
            start_game.emit(GameDifficulty::Medium);
        })
    };

    let start_hard_game = {
        let start_game = start_game.clone();
        Callback::from(move |_| {
            start_game.emit(GameDifficulty::Hard);
        })
    };

    html! {
      <>
        <div class="row h-100 align-items-center justify-content-center">
          <div class="col-auto">
            <div class="row">
              <div class="col">
                <h1>{ "Welcome to the W"}<span class="text-warning">{"o"}</span><span class="text-success">{"r"}</span>{"d Game" }</h1>
                </div>
            </div>
            <div class="row justify-content-center">
              <div class="col-auto">
                <h3>{ "Select your Difficulty" }</h3>
              </div>
            </div>
            <div class="row justify-content-center">
              <div class="col col-md-6">
                    <button class="btn btn-lg btn-success w-100 mb-2" onclick={start_easy_game}>{ "Easy" }</button>
                    <button class="btn btn-lg btn-warning w-100 mb-2" onclick={start_medium_game}>{ "Medium" }</button>
                    <button class="btn btn-lg btn-danger w-100" onclick={start_hard_game}>{ "Hard" }</button>
              </div>
            </div>
          </div>
        </div>
      </>
    }
}
