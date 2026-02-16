use super::Solution;

impl Solution {
    pub fn find_coins(num_ways: Vec<i32>) -> Vec<i32> {
        let mut dp = vec![0; num_ways.len() + 1];
        dp[0] = 1;
        for (index, &num_way) in num_ways.iter().enumerate() {
            dp[index + 1] = num_way;
        }

        let mut ret: Vec<i32> = vec![];
        for i in 1.. dp.len() {
    
            if dp[i] == 1 {
                ret.push(i as i32);
                
                for j in (i..dp.len()).rev() {
                    dp[j] -= dp[j - i];
                    if dp[j] < 0 {
                    
                        return vec![];
                    }
                }
            } else if dp[i] > 1{
            
                return vec![];
            }
        }

        ret
    }
}
