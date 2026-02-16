use std::vec;

use super::Solution;

impl Solution {
    pub fn num_of_subsequences(s: String) -> i64 {
        let (mut ls, mut ts): (i64, i64) = (0, 0);
        let mut l_counts = vec![0; s.len()];
        let mut t_counts = vec![0; s.len()];
        let mut c_position = vec![];

        for (index, c) in s.char_indices().filter(|&(_, c)| c == 'L' || c == 'C' || c == 'T') {
            if c == 'L' { ls += 1 } else if c == 'T' { ts += 1; } else if c == 'C'{ c_position.push(index); }
            l_counts[index] = ls;
            t_counts[index] = ts;
        }
        // println!("t_counts = {:?}", t_counts);
        // let t_sum = t_counts.last().copied().unwrap();
        // println!("t_sum = {}", t_sum);
        for  i in 0..t_counts.len() {
            t_counts[i] = ts - t_counts[i];
        }

        let mut max_addition = 0;
        for  i in 0..t_counts.len() {
            max_addition = max_addition.max(l_counts[i] * t_counts[i]);
        }
        //println!("max addition = {}", max_addition);

        let first: i64 = c_position.iter().map(|&i| {
            (l_counts[i] + 1) * t_counts[i]
        }).sum();
        let second: i64 = c_position.iter().map(|&i| {
            (t_counts[i] + 1) * l_counts[i]
        }).sum();
        let third: i64 = c_position.iter().map(|&i| {
            l_counts[i] * t_counts[i]
        }).sum::<i64>() + max_addition;

        //println!("first, second, third = {}, {}, {}", first, second, third);
        first.max(second).max(third)
    }
}
