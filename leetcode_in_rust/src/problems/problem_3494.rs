use std::vec;

use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3494
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let wizard = skill.len();

        let mut prev: Vec<i64> = vec![0; wizard + 1];
        for (_i, &m) in mana.iter().enumerate() {
            let mut cur: Vec<i64> = vec![0; wizard + 1];
            let mut acc = 0;
            for (j, &s) in skill.iter().enumerate() {
                acc += m as i64 * s as i64;
                cur[j + 1] = acc;
            }
            let min_start_time = (0..wizard)
                .map(|j| prev[j + 1] - cur[j])
                .max()
                .unwrap_or(0)
                .max(0);
            cur.iter_mut().for_each(|x| *x += min_start_time);
            std::mem::swap(&mut cur, &mut prev); 
            
        }

        prev[wizard]
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_time(vec![1,5,2,4], vec![5,1,4,2]);
        assert_eq!(110, ret);
    }
}