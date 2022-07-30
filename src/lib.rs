mod game;
use game::{logic, Guess};
extern crate serde_json;

use serde_json::json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_word() -> String {
    logic::pick_word()
}

#[wasm_bindgen]
pub fn get_suggestion(input: &str) -> String {
    // parse input as JSON string into a Vector of guess
    let guesses = serde_json::from_str::<Vec<Guess>>(input).unwrap();
    let (suggestion, probability) = logic::create_suggestion(&guesses);
    // return suggestion and probability as a JSON string
    json!({
        "suggestion": suggestion,
        "probability": probability,
    })
    .to_string()
}

#[wasm_bindgen]
pub fn get_feedback(input: &str, stored_word: &str) -> String {
    let feedback = logic::compare_input(&input, &stored_word);
    // convert the feedback to a json string
    serde_json::to_string(&feedback).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Feedback;
    use serde::Deserialize;
    #[derive(Deserialize, Debug)]
    struct Suggestion {
        suggestion: String,
        probability: f32,
    }

    #[test]
    fn test_get_word() {
        let word = get_word();
        assert_eq!(word.len(), 5);
    }
    #[test]
    fn test_get_suggestion() {
        let word = get_word();
        let guess = Guess {
            word: word.clone(),
            feedback: vec![],
        };
        let guesses: Vec<Guess> = vec![guess];
        // convert guesses to json string
        let guesses_json = serde_json::to_string(&guesses).unwrap();
        let result = get_suggestion(&guesses_json);
        let suggestion = serde_json::from_str::<Suggestion>(&result);
        assert!(suggestion.is_ok());
        // if suggestion is ok
        let suggestion = suggestion.unwrap();
        assert_eq!(suggestion.suggestion.len(), 5);
        assert_eq!(suggestion.probability > 0 as f32, true);
    }
    #[test]
    fn test_get_feedback() {
        let feedback = get_feedback(&"aa", &"aa");
        // convert feedback to json object
        let feedback_json = serde_json::from_str(&feedback).unwrap();
        // convert feedback_json to vector of Feedback objects
        let feedback_vec: Vec<Feedback> = serde_json::from_value(feedback_json).unwrap();

        feedback_vec
            .into_iter()
            .for_each(|item: Feedback| assert_eq!(item.in_correct_location, true));
        let feedback = get_feedback(&"abc", &"acd");
        let feedback_json = serde_json::from_str(&feedback).unwrap();
        let feedback_vec: Vec<Feedback> = serde_json::from_value(feedback_json).unwrap();
        feedback_vec.into_iter().for_each(|item: Feedback| {
            if item.letter == 'a' {
                assert_eq!(item.in_correct_location, true);
            } else {
                assert_eq!(item.in_correct_location, false);
            }
            if item.letter != 'b' {
                assert_eq!(item.in_word, true);
            } else {
                assert_eq!(item.in_word, false);
            }
            if item.letter == 'c' {
                assert_eq!(item.in_word, true);
            }
        });
    }
}
