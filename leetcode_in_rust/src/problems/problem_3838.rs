use std::{collections::HashMap, hash::Hash, usize};

use super::Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words.into_iter().map(|word| {
            let sum = word.chars().map(|ch| {
                weights[(ch as u8 - 'a' as u8) as usize]
            }).sum::<i32>();

            ((sum % 26 + 26) % 26) as u8
        }).map(|idx| {
            ('z' as u8 - idx) as char
        }).collect::<String>()
    }


    pub fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
        let mut occurance: HashMap<String, usize> = HashMap::new();
        let k = k as usize;

        words.into_iter().for_each(|word| {
            if word.len() < k {
                return;
            }
            let prefix = word[0..k].to_string();
            *occurance.entry(prefix).or_insert(0) += 1;
        });

        occurance.into_values()
            .map(|freq| if freq >= 2 {1} else { 0 } )
            .sum::<i32>()
    }
}
