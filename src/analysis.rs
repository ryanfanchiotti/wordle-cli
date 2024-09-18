/* 
for each guess:
    check how many other words it would remove if the answer was each possible answer
    create table of removals for each letter in each space
    average the amount of removals
display:
    rank possible words
    show best possible word
multithreading (?)
*/

use std::cmp::min;

// check if a word is possible after a guess (assuming the correct answer)
fn is_possible(guess: &str, left: &str, answer: &str) -> bool {
    // filtering criteria: letter count and exact matches
    for (i, (guess_letter, ans_letter)) in guess.chars().zip(answer.chars()).enumerate() {
        // exact match and char is not present at index in left
        if guess_letter == ans_letter && left.chars().nth(i).unwrap() != guess_letter { return false; }
        // if min(guess letter count, ans letter count) > answer letter count, left is removed
        if min(
            guess.chars().filter(|chr| *chr == guess_letter).count(),
            answer.chars().filter(|chr| *chr == guess_letter).count()
        ) > left.chars().filter(|chr| *chr == guess_letter).count() {  return false; }
    }
    
    true
}

pub struct WordleAnalyzer {
    all_words: Vec<String>,
    current_words: Vec<String>,
}

impl WordleAnalyzer {
    pub fn new(total_words: Vec<String>, initial_words: Vec<String>) -> WordleAnalyzer {
        WordleAnalyzer{ 
            all_words: total_words,
            current_words: initial_words,
        }
    }
    
    // map each possible guess to possible words left and return amount of guesses that beat input guess
    pub fn guess(&self, word: String, answer: String) -> usize {
        let mut guess_scores: Vec<usize> = Vec::new();
        let mut left: usize = 0;
        
        for possible_guess in &self.all_words {
            let mut guess_score: usize = 0;
            for possible_left in &self.current_words {
                // check how many words are possible answers after guess for each possible word
                guess_score += is_possible(possible_guess, possible_left, &answer) as usize;
            }
            guess_scores.push(guess_score);
            if possible_guess == &word {
                left = guess_score;
            }
        }
        
        guess_scores.iter().filter(|still_left| **still_left < left).count()
    }
    
    pub fn filter_words(&mut self, guess: String, answer: &str) {
        self.current_words = self.current_words
                                .iter()
                                .filter(|word| is_possible(&guess, word, answer))
                                .cloned()
                                .collect();
    }
}