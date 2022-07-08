// Create a game which selects a random 5 letter word from a bank
// User provides a 5 letter word input from the console
// Check the input for matches against the randomly selected word
// Ouput the word back to the user with colored formatting
// If the letter occurs in the word, it should be colored yellow
// If the Letter occurs in the word, and is in the correct location, it should be colored green
// If the input matches the word exactly, the user wins!

// use std::io;

// use colored::{ColoredString, Colorize};

use super::{Feedback, Guess};
use rand::Rng;
const WORDS: [&str; 55] = [
    "apple", "cargo", "fishy", "grape", "honey", "juice", "lemon", "mango", "peach", "shell",
    "piece", "owned", "pepsi", "peace", "grand", "queen", "large", "screw", "taken", "shame",
    "beard", "knave", "upset", "orbit", "moxie", "skunk", "grove", "stain", "swing", "snail",
    "prime", "rally", "devil", "jesus", "haire", "drift", "crazy", "cloud", "snarl", "force",
    "greet", "crowd", "mourn", "chant", "retch", "equal", "inlay", "favor", "grace", "march",
    "surge", "buggy", "poppy", "kevin", "sleep",
];

pub fn pick_word() -> String {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..WORDS.len());
    WORDS[random_index].to_string()
}

pub fn get_doubles(stored_word: &str) -> Vec<char> {
    // Method to return a vector of letters which occur more than once in stored_word
    let mut doubles: Vec<char> = Vec::new();
    let mut chars: Vec<char> = stored_word.chars().collect();
    chars.sort();
    for i in 0..(chars.len() - 1) {
        if doubles.contains(&chars[i]) {
            continue;
        }
        if chars[i] == chars[i + 1] {
            doubles.push(chars[i]);
        }
    }
    doubles
}

pub fn compare_input(input: &str, stored_word: &str) -> Vec<Feedback> {
    let mut feedback = Vec::new();
    let double_letters = get_doubles(&stored_word);
    for (i, c) in input.to_lowercase().chars().enumerate() {
        let in_word = stored_word.contains(c);
        let in_correct_location = stored_word.chars().nth(i).unwrap() == c;
        feedback.push(Feedback {
            letter: c,
            in_word: in_word,
            in_correct_location: in_correct_location,
            possible_double: double_letters.contains(&c),
        });
    }
    feedback
}

pub fn create_suggestion(guesses: &Vec<Guess>) -> (String, f32) {
    // Method to check the current guesses and suggest the best answer based on feedback
    // Filter the WORDS based on the feedback in the guesses vector
    let mut filtered_words = WORDS.to_vec();
    for guess in guesses {
        let feedback = guess.feedback.clone();
        let mut filtered_words_clone = filtered_words.clone();
        for (indx, f) in feedback.iter().enumerate() {
            if f.in_correct_location {
                filtered_words_clone.retain(|x| x.chars().nth(indx).unwrap() == f.letter);
            } else if f.in_word {
                filtered_words_clone
                    .retain(|w| w.contains(f.letter) && w.chars().nth(indx).unwrap() != f.letter);
            } else {
                filtered_words_clone.retain(|w| !w.contains(f.letter));
            }
        }
        filtered_words = filtered_words_clone;
    }
    // Pick a random word from the filtered words vector
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..filtered_words.len());
    let filter_length: f32 = filtered_words.len() as f32;
    (
        filtered_words[random_index].to_string(),
        ((1 as f32 / filter_length) * 100 as f32) as f32,
    )
}

// fn display_result(input: &str, feedback: &Vec<Feedback>) -> bool {
//     // if all items in_correct_location is true, then the user won!
//     let mut user_won = true;
//     for f in feedback {
//         if !f.in_correct_location {
//             user_won = false;
//         }
//     }
//     if user_won {
//         println!(
//             "{} is correct! You won!",
//             format!("{}", input).green().bold()
//         );
//         return true;
//     }
//     // if in_correct_location is true, letter should be green
//     // if in_word is true, letter should be yellow
//     fn format_letter(f: &Feedback) -> ColoredString {
//         if f.in_correct_location {
//             format!("{}", f.letter).green()
//         } else if f.in_word {
//             format!("{}", f.letter).yellow()
//         } else {
//             format!("{}", f.letter).red()
//         }
//     }

//     println!(
//         "{}{}{}{}{}",
//         format_letter(&feedback[0]),
//         format_letter(&feedback[1]),
//         format_letter(&feedback[2]),
//         format_letter(&feedback[3]),
//         format_letter(&feedback[4])
//     );
//     false
// }

// fn game() {
//     println!("Welcome to the word game!");
//     loop {
//         // pick a random word
//         let word = pick_word();
//         println!("Your word has been chosen. Please enter a 5 letter word.");
//         loop {
//             let mut input = String::new();
//             io::stdin()
//                 .read_line(&mut input)
//                 .expect("Failed to read line");

//             // trim the input
//             input = input.trim().to_string();

//             if input.len() != 5 {
//                 println!("Please enter a 5 letter word.");
//             } else {
//                 // check the input for correctness
//                 let feedback = compare_input(&input, &word);
//                 let won = display_result(&input, &feedback);
//                 if won {
//                     break;
//                 }
//             }
//         }
//         // Ask if the user wants to play again
//         println!("Would you like to play again? (y/n)");
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         if input.trim() == "n" {
//             println!("Thanks for playing!");
//             break;
//         }
//     }
// }
