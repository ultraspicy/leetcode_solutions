use super::Solution;
use std::collections::HashMap;

impl Solution {
    // TODO: Implement the method for problem 3289
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }

        map.iter()
        // filter method passes references to these pairs to its closure
        .filter(|(_, &count)| count == 2)
        .map(|(&key, _)| key)
        .collect()
    }
}
