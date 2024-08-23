
use super::Solution;

use std::{collections::HashMap, hash::Hash};

impl Solution {
    pub fn winning_player_count(n: i32, picks: Vec<Vec<i32>>) -> i32 {
        let mut map:HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for pick in picks {
            let color_frequency = map.entry(pick[0]).or_insert(HashMap::new());
            let _ = *color_frequency.entry(pick[1]).or_insert(0) + 1; 
        }

        let win = map.iter().
            filter(
                |(&k, v)| v.values().max().map_or(false, |&max| max > k + 1)).
            count() as i32;

        n - win
    } 
}  