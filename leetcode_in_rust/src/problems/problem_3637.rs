use super::Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut p = 0;
        let mut q = nums.len() - 1;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                p = i;
            } else {
                break;
            }
        }
        if p >= nums.len() - 2 || p == 0 {
            return false;
        }

        for i in (1..nums.len()).rev() {
            if nums[i] > nums[i - 1] {
                q = i - 1;
            } else {
                break;
            }
        }

        if q <= p || q == nums.len() - 1{
            return false;
        }

        for i in (p + 1)..=q {
            if nums[i] >=  nums[i - 1] {
                return false;
            }
        }

        true
    }
}
