use super::Solution;
use std::collections::{HashMap, HashSet};

impl Solution {
    
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        s1
            .split_whitespace()
            .chain(s2.split_whitespace())
            .fold(HashMap::new(), |mut acc, word| {
                *acc.entry(word).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .filter(|(k, v)| *v == 1) // filter borrows the item so (k, v) are refs
            .map(|(k, v)| k.to_string()) // map moves the item, so (k, v) are owned values 
            .collect::<Vec<_>>()
    }
}
