use super::Solution;
// this file is for playground purpose since 
// not all questions are worth to be saved
impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
       (left..=right)
            .map(|n| n.count_ones())
            .filter( |n|
                match n {
                    2 | 3 | 5 | 7 | 11 |13 | 17 | 19 => { true },
                    _ => false,
                }
            )
            .count() as i32
    }
}
