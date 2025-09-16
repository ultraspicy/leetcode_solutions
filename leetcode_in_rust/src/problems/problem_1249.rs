use std::{collections::HashSet, vec};

use super::Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut valid_left = HashSet::new();
        let mut valid_right = HashSet::new();
        let mut lefts = vec![];

        for (i, c) in s.char_indices() {
            if c.is_ascii_alphabetic() {
                // nothing
            } else if c == '(' {
                lefts.push(i);
            } else {
                if let Some(last) = lefts.pop() {
                    valid_left.insert(last);
                    valid_right.insert(i);
                }
            }
        }

        s.char_indices().filter_map(|(i, c)| {
            if c == '(' && valid_left.contains(&i) {
                Some(c)
            } else if c == ')' && valid_right.contains(&i) {
                Some(c)
            } else if c.is_ascii_alphabetic() {
                Some(c)
            } else {
                None
            }
        }).collect::<String>()
    }
}
