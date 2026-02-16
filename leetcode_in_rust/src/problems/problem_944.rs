use super::Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        (0..strs[0].len())
            .map(|i| {
                strs.iter()
                    .map(|s| s.chars().nth(i).unwrap())
                    .collect::<Vec<_>>()
                    .windows(2)
                    .all(|pair| pair[0] <= pair[1])
            })
            .map(|ascending | {
                if !ascending {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>()
    }
}
