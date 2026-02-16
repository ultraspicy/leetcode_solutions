use super::Solution;

impl Solution {
    
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = nums.iter().sum();
        let mut divider = nums.len();
        let mut ret: i32 = 0;

        for num in nums.into_iter() {
            sum = sum - num;
            divider -= 1;

            if (sum as f32 / divider as f32) < num as f32 {
                ret += 1;
            }
        }
        ret
    }
}
