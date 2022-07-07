use yew::{function_component, html, use_effect_with_deps, use_state, Properties};

#[derive(Properties, PartialEq)]
pub struct SuggestionProps {
    pub suggestion: String,
}

#[function_component(Suggestion)]
pub fn suggestion(SuggestionProps { suggestion }: &SuggestionProps) -> Html {
    // use_state for tracking whether the suggestion is visible or obscured
    let is_visible = use_state(|| false);
    // use_state for tracking the class on the suggestion div
    let class_name = use_state(|| "suggestion-hidden");

    // handler to toggle visibility
    let toggle_visibility = {
        let is_visible = is_visible.clone();
        move |_| {
            is_visible.set(!*is_visible);
        }
    };

    use_effect_with_deps(
        {
            let is_visible = is_visible.clone();
            move |_| {
                is_visible.set(false);
                || ()
            }
        },
        (*suggestion).clone(),
    );

    // use_effect_with_deps to change the class_name based on is_visible
    use_effect_with_deps(
        {
            let class_name = class_name.clone();
            let is_visible = is_visible.clone();
            move |_| {
                if *is_visible {
                    class_name.set("suggestion-visible");
                } else {
                    class_name.set("suggestion-hidden");
                }
                || ()
            }
        },
        (*is_visible).clone(),
    );

    html! {
        <>
            <div class="row justify-content-center mt-2 mb-2 cursor-pointer" onclick={toggle_visibility}>
              <div class="col-auto">
                <h5 class={*class_name}><span>{ &*suggestion.to_uppercase() }</span></h5>
              </div>
            </div>
        </>
    }
}
