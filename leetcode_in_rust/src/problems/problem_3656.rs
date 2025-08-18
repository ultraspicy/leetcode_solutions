use std::vec;

use super::Solution;

impl Solution {
    pub fn simple_graph_exists(mut degrees: Vec<i32>) -> bool {
        // Erdős–Gallai theorem
        let sum :i32 = degrees.iter().sum(); // overflow won't change parity
        if sum % 2 == 1 { return false; }

        degrees.sort();
        let degrees = degrees.into_iter().map(|x| x as i64).rev().collect::<Vec<i64>>();
        let len = degrees.len();
        let mut prefix_sum: Vec<i64> = vec![];
        let mut sum = 0;
        for i in 0.. degrees.len() {
            prefix_sum.push(sum);
            sum += degrees[i] as i64;     
        }
        prefix_sum.push(sum);
        //println!("prefix_sum = {:?}", prefix_sum);

        let mut left: i64 = 0;
        for i in 0..degrees.len() {
            left += degrees[i] as i64;
            
            let i_i64 = i as i64;
            let flat_index = Self::binary_search_3656(&degrees, i + 1, i_i64 + 1);
            let right = i_i64 * (i_i64 + 1) + prefix_sum[len] - prefix_sum[flat_index] + (i_i64 + 1) * (flat_index as i64 - i_i64 - 1);
            if left > right {
                return false;
            } 
        }
        
        true
    }


    // find the first index i so that vec[i] <= target
    // assume the input is in descending order
    fn binary_search_3656<T: Ord>(vec: &Vec<T>, from: usize, target: T) -> usize {
        if from == vec.len() {
            return vec.len();
        }
        if vec[from] <= target {
            return from;
        }
        let (mut l, mut r) = (from, vec.len() - 1);
        while l + 1 < r {
            let mid = (l + r) / 2;
            if vec[mid] > target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if vec[l] <= target { l } else if vec[r] <= target { r } else { vec.len() }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn unit_test() {
        let vec = vec![8, 7, 6, 5, 4];
        let ret = Solution::binary_search_3656(&vec, 1, 5);
        assert_eq!(ret, 3);
    }

     #[test]
    fn unit_test2() {
        let vec = vec![8, 7, 6, 5, 4];
        let ret = Solution::binary_search_3656(&vec, 1, 2);
        assert_eq!(ret, 5);
    }

    #[test]
    fn unit_test3() {
        let ret = Solution::simple_graph_exists(vec![1, 1, 1]);
        assert_eq!(ret, false);
    }
}