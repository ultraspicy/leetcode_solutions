use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        if s == goal {
            return s.chars().collect::<HashSet<_>>().len() < s.len()
        }
        
        let diffs: Vec<(char, char)> = s
            .chars()
            .zip(goal.chars())
            .filter(|(a, b)| a != b)
            .collect();
    
        diffs.len() == 2 && diffs[0].0 == diffs[1].1 && diffs[0].1 == diffs[1].0
    }
}
