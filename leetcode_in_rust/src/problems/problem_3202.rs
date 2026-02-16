use std::vec;

use super::Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 2;
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];

        nums.iter().enumerate().for_each(|(_, num)|{
            let n = *num as usize % k;
            for i in 0..k {
                dp[i][n] = dp[n][i] + 1;
                ret = ret.max(dp[i][n]); 
            }
        });

        ret
    }
}
