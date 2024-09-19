use std::cmp::min;
use itertools::izip;
use rayon::prelude::*;

// check if a word is possible after a guess (assuming the correct answer)
fn is_possible(guess: &str, left: &str, answer: &str) -> bool {
    // filtering criteria: letter count and exact matches
    for (guess_letter, ans_letter, left_letter) in izip!(guess.chars(), answer.chars(), left.chars()) {
        // guess and answer match, not present at index in left
        if guess_letter == ans_letter && left_letter != guess_letter { return false; }
        // left and guess match, not present at index in answer
        if left_letter == guess_letter && ans_letter != guess_letter { return false; }
        
        let left_chr_cnt = left.chars().filter(|chr| *chr == guess_letter).count();
        let ans_chr_cnt = answer.chars().filter(|chr| *chr == guess_letter).count();
        let guess_chr_cnt = guess.chars().filter(|chr| *chr == guess_letter).count();
        
        // if min(guess letter count, ans letter count) > left letter count, left is removed
        if min(guess_chr_cnt,ans_chr_cnt) > left_chr_cnt { return false; }
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
        // possible remaining words to pick between for guess
        let left = self.current_words
            .iter()
            .filter(|possible_left| is_possible(&word, possible_left, &answer))
            .count();
        
        // create vector of possible remaining words to pick between for each word
        let guess_scores: Vec<usize> = self.all_words
            .par_iter()
            .map(|possible_guess| self.current_words
                .iter()
                .filter(|possible_left| is_possible(possible_guess, possible_left, &answer))
                .count())
            .collect();
        
        guess_scores.iter().filter(|still_left| **still_left < left).count()
    }
    
    // remove impossible words from pool
    pub fn filter_words(&mut self, guess: String, answer: &str) {
        self.current_words = self.current_words
            .iter()
            .filter(|word| is_possible(&guess, word, answer))
            .cloned()
            .collect();
    }
}