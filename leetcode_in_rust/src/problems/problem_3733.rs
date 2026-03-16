use super::Solution;

impl Solution {
    pub fn minimum_time(d: Vec<i32>, r: Vec<i32>) -> i64 {
        
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
}
