use std::{collections::HashMap, vec};

use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3837
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn delayed_count(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut idx: HashMap<i32, Vec<usize>> = HashMap::new();
        let len = nums.len();

        nums.into_iter()
            .enumerate()
            .for_each(|(i, num)|{
                idx.entry(num).or_insert(Vec::new()).push(i);
            });

        let mut ret: Vec<i32> = vec![0; len];
        println!("idx = {:?}", idx);
        
        for key in idx.keys() {
            let val = idx.get(key).unwrap(); // {0, 2, 3}
            for i in 0..val.len() {
                let first_valid_position = val[i] + k as usize;
                let insert_position = val.partition_point(|&x| x <= first_valid_position);
                ret[val[i]] = (val.len() - insert_position) as i32;
                println!("val[i] = {}, insert_position = {}", val[i], insert_position);
            }
        }

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::delayed_count(vec![1,2,1,1],  1);
        println!("{:?}", ret);
    }
}