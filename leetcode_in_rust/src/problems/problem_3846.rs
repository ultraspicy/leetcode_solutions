use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn total_distance(s: String) -> i32 {
        let alpha: [&str; 3] =["qwertyuiop", "asdfghjkl", "zxcvbnm"];
        let mut row_idx = HashMap::new();
        let mut col_idx = HashMap::new();
        alpha.into_iter().enumerate()
            .for_each(|(i, s)| {
                s.chars().for_each(|c| { row_idx.insert(c, i);});
            });

        for i in 0..alpha[0].len() {
            let c = alpha[0].chars().nth(i).unwrap(); // unwrap since we know it exists
            col_idx.insert(c, i);

            if let Some(c) = alpha[1].chars().nth(i) {
                col_idx.insert(c, i);
            }

            if let Some(c) = alpha[2].chars().nth(i) {
                col_idx.insert(c, i);
            }
        }

        let mut cur = (1, 0);
        let mut ret = 0;
        for c in s.chars() {
            let new = (row_idx[&c] as i32, col_idx[&c] as i32);
            ret += (cur.0 - new.0).abs() + (cur.1 - new.1).abs();
            cur = new;
        }

        ret
    }
}
