use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3499
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut flag = vec![1; n];
        let mut acc = 1;
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                acc += 1;
                flag[i] = acc;
            } else {
                acc = 1;
            }
            if acc >= k && i as i32 - k >= 0 && flag[i - k as usize] >= k  {
                return true;
            }
        }

        false
    }
}
