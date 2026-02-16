use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let to_dictionary = |s: &str| s.chars()
            .fold(HashMap::new(), |mut acc: HashMap<char, i32>, ch| {
                *acc.entry(ch).or_insert(0) += 1;
                acc
        });

        let chars_dictionary = to_dictionary(&chars);

        words.iter()
            .filter(|&word| {
                let word_dictionary = to_dictionary(word);
                word_dictionary.iter()
                    .all(|(k, &v)| {
                        match chars_dictionary.get(k) {
                            None => false,
                            Some(&val) => { v <= val}
                        }
                    })
            })
            .map(|word| word.len() as i32)
            .sum()
    }
}
