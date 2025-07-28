use std::collections::{HashMap, HashSet, VecDeque};

use super::Solution;

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        let mut pre_processing = HashMap::new();
        nums.iter().for_each(|&num| {
            if !pre_processing.contains_key(&num) && Self::is_prime_3926(num) {
                for i in 0..nums.len() {
                    if nums[i] % num == 0 {
                        pre_processing.entry(num).or_insert_with(Vec::new).push(i);
                    }
                }
            }
        });

        let mut q = VecDeque::new();
        let mut visited = vec![false; nums.len()];
        q.push_back((0,0));
        visited[0] = true;

        while let Some((index, jump)) = q.pop_front() {
            // if index == nums.len() - 1 {
            //     return jump;
            // }
            if let Some(index_divide_prime) = pre_processing.get(&nums[index]) {
                for &i_divide_prime in index_divide_prime {
                    if !visited[i_divide_prime] && i_divide_prime != index{
                        q.push_back((i_divide_prime, jump + 1));
                        visited[i_divide_prime] = true;
                        if i_divide_prime == nums.len() - 1{
                            return jump + 1;
                        }
                    }
                }
            }
            if index > 0 && !visited[index - 1] {
                q.push_back((index - 1, jump + 1));
                visited[index - 1] = true;
                
            } 
            if index < nums.len() - 1 && !visited[index + 1] {
                q.push_back((index + 1, jump + 1));
                visited[index + 1] = true;
                if index + 1 == nums.len() - 1{
                    return jump + 1;
                }
            }
        }

       0
    }

    fn is_prime_3926(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        if num == 2 || num == 3 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }

        let sqrt_num = (num as f64).sqrt() as i32;
        for i in (3..=sqrt_num).step_by(2) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_jumps(vec![2,3,4,7,9]);
        assert_eq!(ret, 2)
    }

    #[test]
    fn unit_test2() {
        let ret = Solution::min_jumps(vec![4,6,5,8]);
        assert_eq!(ret, 3)
    }
}