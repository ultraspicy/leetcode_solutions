use super::Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let len = fruits.len();
        let mut basket_taken = vec![false; len];

        let mut ret = 0;
        for i in 0..len {
            for j in 0..len {
                if !basket_taken[j] && fruits[i] <= baskets[j] {
                    basket_taken[j] = true;
                    ret += 1;
                    break;
                }
            }
        }

       len as i32 - ret
    }
}
