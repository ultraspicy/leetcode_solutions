use super::Solution;

impl Solution {
    pub fn maximum_median_sum(nums: Vec<i32>) -> i64 {
        let mut nums_mut = nums.clone();
        nums_mut.sort_unstable();

        nums_mut.into_iter()
            .rev()
            .map(|x| x as i64)
            .skip(1)
            .step_by(2)
            .take(nums.len() / 3)
            .sum()
    }
}
