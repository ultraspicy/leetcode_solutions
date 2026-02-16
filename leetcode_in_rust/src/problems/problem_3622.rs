use super::Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let digits: Vec<i32> = n
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as i32)
            .collect();

        let sum: i32 = digits.iter().sum();
        let product: i32 = digits.iter().product();

        n % (sum + product) == 0
    }
}
