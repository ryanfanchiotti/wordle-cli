mod colors;
use std::io::{stdin, stdout, Write};
use colors::*;

// find green (exact match) and yellow (partial match) letters in current word
fn word_cmp(cur_word: &str, target_chars: &Vec<char>) -> String {   
    let mut output = String::new();
    
    for (letter, target_letter) in cur_word.chars().zip(target_chars.into_iter()) {
        if letter == *target_letter { output += GREEN_BOLD; }
        else if target_chars.contains(&letter) { output += YELLOW_BOLD; }
        else { output += RED_BOLD; }
        output.push(letter);
        output += RESET;
        output.push(' ');
    }
    output.trim_end().to_string()
}

// run main wordle simulation and parse input
fn run_wordle(target_word: String, guesses: usize) -> Vec<String> {
    let mut input = String::new();
    let mut answers = Vec::new();
    let target_size = target_word.len();
    let target_chars: Vec<char> = target_word
        .chars()
        .collect();
            
    for guess_num in 1..=guesses {
        print!("{BLUE_BOLD}Guess {guess_num}:{RESET} ");
        stdout().flush().expect("Standard output flush failed");
        input.clear();
        
        // repeat input until guess is correct size
        while input.len() != target_size {
            input.clear();
            stdin().read_line(&mut input)
                .unwrap_or_else(|err| {println!("String input error: {err}"); 0});
            input = input.trim()
                .to_ascii_uppercase()
                .chars()
                .filter(|ch| ch.is_ascii_alphabetic())
                .collect();
                
            // prompt for guess if incorrect amount of letters
            if input.len() != target_size {
                println!("{CYAN_BOLD}Letter amount mismatch, expected: {target_size}{RESET}");
                print!("{BLUE_BOLD}Guess {guess_num}:{RESET} ");
                stdout().flush().expect("Standard output flush failed");
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

fn main() {
    let _answers = run_wordle("CRANE".to_string(), 6);
}
