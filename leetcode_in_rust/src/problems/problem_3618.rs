use super::Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        let mut sum: i64 = 0;

        for i in 0.. nums.len() {
            if Self::is_prime(i) {
                sum += *nums.get(i).unwrap() as i64;
            } else {
                sum -= *nums.get(i).unwrap() as i64;
            }
        }

        sum.abs() as i64
    }


    fn is_prime (n : usize) -> bool {
        if n == 0 || n == 1 {
            return false
        }

        if n == 2 || n == 3 {
            return true;
        }

        if n % 2 == 0 {
            return false;
        }

        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i +=1;
        }

        return true;
    }
}
