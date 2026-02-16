use std::collections::VecDeque;

use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3835
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();
        // minimum number along the iteration
        // smaller number comes after, then it annilates its previous
        // min_q is increasing, min is first() 
        let mut min_q: VecDeque<usize> = VecDeque::new();
        // maximum number along the iteration
        // greater number comes after, then it annilates its previous 
        // max_q is decreasing , max is first()
        let mut max_q: VecDeque<usize> = VecDeque::new();

        let mut ret: i64 = 0;
        let mut l = 0;
        for i in 0..n {
            let cur = nums[i];
            while let Some(min_idx) = min_q.back().copied() {
                if cur <= nums[min_idx] {
                    min_q.pop_back();
                } else {
                    break;
                }
            }
            min_q.push_back(i);

            while let Some(max_idx) = max_q.back().copied() {
                if cur >= nums[max_idx] {
                    max_q.pop_back();
                } else {
                    break;
                }
            }
            max_q.push_back(i);

            while (nums[max_q.front().copied().unwrap()] - nums[min_q.front().copied().unwrap()]) as i64 * (i - l + 1) as i64 > k {
                if max_q.front().copied().unwrap() == l {
                    max_q.pop_front();
                }
                if min_q.front().copied().unwrap() == l {
                    min_q.remove(0);
                }
                l += 1;
            }
            ret += (i - l + 1) as i64;
        }

        ret
    }
}
