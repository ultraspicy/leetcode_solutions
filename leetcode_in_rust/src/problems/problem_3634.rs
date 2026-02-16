use std::i32;

use super::Solution;

impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut ret = i32::MAX;

        for i in 0..nums.len() {
            let upper_limit = k as i64 * nums[i] as i64;
            if let Some(last_index) = Self::binary_search_last_appearance_at_most_n(&nums, upper_limit) {
                ret = ret.min(i as i32 + nums.len() as i32 - 1 - last_index);
                if ret == 0 {
                    return 0;
                }
            }
        }
        ret
    }

    fn binary_search_last_appearance_at_most_n(nums: &Vec<i32>, n: i64) -> Option<i32> {
        let (mut start, mut end) = (0, nums.len() - 1);
        while start + 1 < end {
            let mid = (start + end) / 2;
            if nums[mid] as i64 <= n {
                start = mid;
            } else {
                end = mid;
            }
        }
        if nums[end] as i64 <= n {
            return Some(end as i32);
        } else if nums[start] as i64 <= n {
            return Some(start as i32);
        } else {
            return None;
        }
    }
}
