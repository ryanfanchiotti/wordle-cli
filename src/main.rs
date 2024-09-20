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
    NORMAL_BOLD,
    RESET
};
use analysis::WordleAnalyzer;
use chrono::prelude;
use reqwest::blocking;
use std::collections::{HashMap, HashSet};

// find green (exact match) and yellow (partial match) letters in current word
fn word_cmp(cur_word: &str, target_chars: &Vec<char>) -> String {   
    let mut output = String::new();
    let mut chars_count: HashMap<char, usize> = HashMap::new();
    
    // count is kept track of here to make sure letters appear yellow as many times as they are in target
    for (letter, target_letter) in cur_word.chars().zip(target_chars.into_iter()) {
        // push letter color and letter to output string
        let letter_count = target_chars
            .iter()
            .filter(|c| **c == letter)
            .count();
        if letter == *target_letter { output += GREEN_BOLD; }
        else if target_chars.contains(&letter) 
            && (!chars_count.contains_key(&letter) || chars_count[&letter] < letter_count) { output += YELLOW_BOLD; }
        else { output += NORMAL_BOLD; }
        
        output.push(letter);
        output += RESET;
        output.push(' ');
        
        // update count of letter
        *chars_count.entry(letter).or_insert(0) += 1;
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
            print!("Guess {guess_num}: ");
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
                println!("{RED_BOLD}Letter amount mismatch, expected {target_size}{RESET}");
                continue;
            }
            
            // check if word is valid
            if !possible_words.contains(&input) {
                println!("{RED_BOLD}Invalid word{RESET}");
                input.clear();
            }
        }
        
        // print current word with green / yellow / red coloring
        println!("-------------");
        let output_str = word_cmp(&input, &target_chars); 
        println!("| {output_str} |");
        println!("-------------");
        
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
    
    // include all words when compiling
    let pos_ans_filestr = include_str!("../words/possible_answers.txt");
    let pos_word_filestr = include_str!("../words/possible_words.txt");
    
    // make hash sets from possible answers and words text files
    let mut possible_answers: HashSet<String> = pos_ans_filestr
        .split("\n")
        .map(|s| s.to_uppercase())
        .collect();
    let mut possible_words: HashSet<String> = pos_word_filestr
        .split("\n")
        .map(|s| s.to_uppercase())
        .collect();
        
    // solutions are stored by the day
    let current_day = prelude::Utc::now().format("%Y-%m-%d").to_string();
    println!("{NORMAL_BOLD}Wordle for {current_day}:{RESET}");
        
    // find current word and insert it into word lists
    let current_word;
    if let Some(word) = get_wordle_word(current_day) {
        possible_words.insert(word.clone());
        possible_answers.insert(word.clone());
        current_word = word.to_uppercase();
    } else {
        println!("There was an issue getting today's Wordle data, please check your internet connection or try again later");
        return; 
    } 
    
    // create vec of answers for analysis
    let answers = run_wordle(current_word.clone(), GUESSES, &possible_words);
    let total_vec: Vec<String> = possible_words.clone().into_iter().collect();
    let possible_vec: Vec<String> = possible_answers.clone().into_iter().collect();
    let mut analyzer = WordleAnalyzer::new(total_vec, possible_vec);
    let total_words = possible_words.len();
    
    if answers.len() == GUESSES && answers.last().unwrap() != &current_word {
        println!("Correct word was: {RED_BOLD}{current_word}{RESET}")
    }
    
    println!("----------------------");
    println!("| Guess | Percentile |");
    println!("----------------------");
    
    for guess in answers {
        let score = analyzer.guess(guess.clone(), current_word.clone());
        let percentile = ((total_words - score) as f64 / total_words as f64) * 100f64;
        let mut spaces: usize = 4;
        if percentile < 10.0 { spaces = 6 }
        else if percentile < 99.9999 { spaces = 5 }
        
        println!("| {guess} | {:.2} {}|", percentile, " ".repeat(spaces));
        analyzer.filter_words(guess.clone(), &current_word);
    }
    println!("----------------------");
}
