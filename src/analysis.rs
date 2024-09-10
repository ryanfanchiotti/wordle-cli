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
    letter_to_removed: HashMap<(char, usize), String>
}

impl WordleAnalyzer {
    pub fn new(total_words: HashSet<String>, initial_words: HashSet<String>) -> WordleAnalyzer {
        let mut letter_pos_map = HashMap::new();
        const LENGTH: usize = 5;
        
        for letter in 65u8..91 {
            for pos in 0..LENGTH{
                for word in &initial_words {
                    
                }
            }
        }
        WordleAnalyzer{ 
            all_words: total_words,
            current_words: initial_words,
            letter_to_removed: letter_pos_map
        }
    }
}