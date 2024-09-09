/* 
for each possible word:
    check how many other words it would remove if the answer was each possible answer
    create table of removals for each letter in each space
    average the amount of removals
display:
    rank possible words
    show best possible word
multithreading (?)
*/

use std::collections::{HashSet, HashMap};

pub struct WordleAnalyzer {
    all_words: HashSet<String>,
    current_words: HashSet<String>,
    guesses: Vec<String>,
    letter_to_removed: HashMap<(char, usize), String>
}

impl WordleAnalyzer {
    pub fn new(total_words: HashSet<String>, initial_words: HashSet<String>, prev_guesses: Vec<String>) -> WordleAnalyzer {
        const GUESSES: usize = 6;
        let mut letter_pos_map = HashMap::new();
        
        for letter in 65u8..91 {
            for pos in 0..5 {
                //println!("letter: {}, pos: {pos}", letter as char)
            }
        }
        WordleAnalyzer{ 
            all_words: total_words,
            current_words: initial_words,
            guesses: prev_guesses,
            letter_to_removed: letter_pos_map
        }
    }
}