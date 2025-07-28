use super::Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max_value = nums.iter().fold(0, |acc, &num| {
            acc | num
        });

        Self::dfs_2044(&nums, 0, 0, max_value)
    }

    fn dfs_2044(nums: &Vec<i32>, cur_index: usize, prev_val:i32, max_value: i32) -> i32 {
        if cur_index == nums.len() {
            if prev_val == max_value { return 1 } else { return 0; }
        }

        let left = Self::dfs_2044(nums, cur_index + 1, prev_val | nums[cur_index], max_value);
        let right = Self::dfs_2044(nums, cur_index + 1, prev_val, max_value);

        left + right
    }
}
