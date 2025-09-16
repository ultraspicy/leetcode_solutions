use super::Solution;

impl Solution {
    pub fn min_operations_1598(logs: Vec<String>) -> i32 {
        logs.iter().map(|str| {
            match str.chars().nth(0) {
                Some('.') => {
                    match str.chars().nth(1)  {
                        Some('.') => -1,
                        _ => 0
                    }
                },
                _ => 1
            }
        })
        .fold(0, |acc, cur| {
            if acc + cur < 0 {
                0
            } else {
                acc + cur
            }
        })
    }
}
