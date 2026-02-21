use std::i32;

use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3844
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn almost_palindromic(s: String) -> i32 {
        let len = s.len();
        let s_vec = s.chars().collect::<Vec<_>>();

        let mut dp = vec![vec![0; len]; len];
        for i in 0..len {
            if i + 1 < len {
                if s_vec[i] == s_vec[i+1] {
                    dp[i][i+1] = 0;
                } else {
                    dp[i][i+1] = 1;
                }
            }
        }

        for offset in 2..len {
            for i in 0..len {
                let j = i + offset;
                if j < len {
                    if s_vec[i] == s_vec[j] {
                        dp[i][j] = dp[i+1][j-1].min((dp[i+1][j] + 1).min(dp[i][j-1] + 1));
                    } else {
                        dp[i][j] = dp[i+1][j].min(dp[i][j-1]) + 1;
                    }
                }
            }
        }
        print!("{:?}", dp);

        let (mut start, mut end) = (0usize, 1usize);
        for i in 0..len {
            for j in (i + 1)..len {
                if dp[i][j] <= 1 && j - i > end - start {
                    start = i;
                    end = j;
                }
            }
        }


        (end - start + 1) as i32
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test() {
        let s = String::from("abba");
        let ret = Solution::almost_palindromic(s);
        assert_eq!(4, ret);
    }
}
