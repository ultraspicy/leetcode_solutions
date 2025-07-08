use super::Solution;
use itertools::Itertools;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        s.chars().group_by(|&x| x)
            .into_iter()
            .map(|(_key, _group)| _group.count())
            .max()
            .unwrap() as i32
    }
}
