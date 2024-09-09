mod colors;
mod analysis;

use std::io::{
    stdin, 
    stdout, 
    Write
};
use colors::{
    GREEN_BOLD,
    YELLOW_BOLD,
    RED_BOLD,
    BLUE_BOLD,
    NORMAL_BOLD,
    RESET
};
use chrono::prelude;
use reqwest::blocking;
use std::fs::read_to_string;
use std::collections::HashSet;

// find green (exact match) and yellow (partial match) letters in current word
fn word_cmp(cur_word: &str, target_chars: &Vec<char>) -> String {   
    let mut output = String::new();
    
    for (letter, target_letter) in cur_word.chars().zip(target_chars.into_iter()) {
        if letter == *target_letter { output += GREEN_BOLD; }
        else if target_chars.contains(&letter) { output += YELLOW_BOLD; }
        else { output += NORMAL_BOLD; }
        output.push(letter);
        output += RESET;
        output.push(' ');
    }
    output.trim_end().to_string()
}

// run main wordle simulation and parse input
fn run_wordle(target_word: String, guesses: usize, possible_words: &HashSet<String>) -> Vec<String> {
    let mut input = String::new();
    let mut answers = Vec::new();
    let target_size = target_word.len();
    let target_chars: Vec<char> = target_word.chars().collect();
            
    for guess_num in 1..=guesses {
        // make sure input is not already a valid guess
        input.clear();
        
        // repeat user input until guess is correct size
        while input.len() != target_size {
            // display guess number
            print!("{BLUE_BOLD}Guess {guess_num}:{RESET} ");
            stdout()
                .flush()
                .expect("Standard output flush failed");
            input.clear();
            
            stdin()
                .read_line(&mut input)
                .unwrap_or_else(|err| {println!("String input error: {err}"); 0});
            input = input
                        .trim()
                        .to_ascii_uppercase()
                        .chars()
                        .filter(|ch| ch.is_ascii_alphabetic())
                        .collect();
                
            // check if incorrect amount of letters
            if input.len() != target_size {
                println!("{RED_BOLD}Letter amount mismatch, expected: {target_size}{RESET}");
                continue;
            }
            
            // check if word is valid
            if !possible_words.contains(&input) {
                println!("{RED_BOLD}Invalid Word{RESET}");
                input.clear();
            }
        }
        
        // print current word with green / yellow / red coloring
        let output_str = word_cmp(&input, &target_chars); 
        println!("{output_str}");
        
        // add current word to vec of answers
        answers.push(input.to_string());
        
        // end cycle at correct guess
        if input == target_word {
            return answers;
        }
    }
    answers
}

// return word associated with today's solution if possible
fn get_wordle_word(current_day: String) -> Option<String> {
    const EXPECTED_SIZE: usize = 5;
    
    let nyt_url = format!("https://www.nytimes.com/svc/wordle/v2/{current_day}.json");
    
    let nyt_text: String;
    if let Ok(nyt_response) = blocking::get(nyt_url) {
        if let Ok(nyt_json_text) = nyt_response.text() {
            nyt_text = nyt_json_text;
        } else { return None; }
    } else { return None; }
    
    if let Ok(data) = json::parse(&nyt_text) {
        let output = data["solution"].to_string().to_uppercase();
        if output.len() == EXPECTED_SIZE {
            Some(output)
        } else { None }
    } else { None }
}

fn main() {
    const GUESSES: usize = 6;
    const _FILENAME: &str = "";
    
    // make hash sets from possible answers and words text files
    let possible_answers: HashSet<String> = read_to_string("words/possible_answers.txt")
        .expect("Possible answers file not found")
        .split("\n")
        .map(|s| s.to_uppercase())
        .collect();
    let possible_words: HashSet<String> = read_to_string("words/possible_words.txt")
        .expect("Possible words to enter file not found")
        .split("\n")
        .map(|s| s.to_uppercase())
        .collect();
        
    // solutions are stored by the day
    let current_day = prelude::Utc::now().format("%Y-%m-%d").to_string();
    println!("{NORMAL_BOLD}Wordle for {current_day}:{RESET}");
    
    let _answers;
    if let Some(word) = get_wordle_word(current_day) {
        _answers = run_wordle(word, GUESSES, &possible_words);
    } else {
        println!("There was an issue getting today's Wordle data from the NYT API, please check your internet connection or try later");
        return; 
    } 
}
