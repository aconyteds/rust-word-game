use yew::{function_component, html, use_state, Properties};

use super::Feedback;

#[derive(Properties, PartialEq)]
pub struct CharacterProps {
    pub character: char,
    pub feedback: Feedback,
}

#[function_component(Character)]
pub fn character(
    CharacterProps {
        character,
        feedback,
    }: &CharacterProps,
) -> Html {
    let text_color = use_state(|| {
        if feedback.in_correct_location {
            return "bg-success text-dark";
        } else if feedback.in_word {
            return "bg-warning text-dark";
        } else {
            return "";
        }
    });

    let double_class = use_state(|| {
        if feedback.possible_double && feedback.in_correct_location {
            return "double";
        } else {
            return "";
        }
    });

    html! {
      <>
        <div class={format!("col-auto border m-1 border-1 border-light {} {}", *text_color, *double_class)}>
            <span class="display-character">{character.to_uppercase()}</span>
        </div>
      </>
    }
}
