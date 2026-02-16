use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3285
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        height
            .iter()
            .enumerate() // enumerate() gives us pairs of (index, &element)
            .take(height.len() - 1)
            // filter takes a reference to each item in the iterator.
            // The outer & is pattern-matching on the reference to the tuple. It's saying "I expect a reference here."
            // filter doesn't change the type of the items in the iterator.
            // Pattern matching on a reference (&) does not move or copy the actual data.
            // It creates new bindings that allow us to access the data through these new names.
            .filter(|&(_i, &h)| h > threshold) 
            .map(|(i, &_h)| i as i32 + 1).collect()
    }
}
