use yew::prelude::*;

mod game;
use crate::game::Game;

#[function_component(App)]
fn app() -> Html {
    html! {
      <main class="h-100 w-100 bg-dark">
        <div class="container h-100">
          <Game />
        </div>
      </main>

    }
}

fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}
