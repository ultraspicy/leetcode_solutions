use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

use super::Solution;

impl Solution {
    pub fn min_jumps(nums: Vec<i32>) -> i32 {
        // let mut min_prime_factor = vec![0; 1000_000 + 1];
        let max_val = *nums.iter().max().unwrap() as usize + 1;
        let mut min_prime_factor = vec![0; max_val];

        for i in 2..min_prime_factor.len() {
            if min_prime_factor[i] != 0 {
                continue;
            }
            for j in ((i * i)..min_prime_factor.len()).step_by(i) {
                if min_prime_factor[j] == 0 {
                    min_prime_factor[j] = i;
                }
            }
        }

        let mut nums_factoried = HashMap::new();
        nums.iter().enumerate().for_each(|(index, &num)| {
            if num == 1 { return; }
            let mut cur = num as usize;
            while min_prime_factor[cur] != 0 {
                let min_factor = min_prime_factor[cur];
                nums_factoried.entry(min_factor).or_insert_with(Vec::new).push(index);
                while  cur % min_factor == 0{
                    cur = cur / min_factor;
                }
            }
            if cur > 1 {
                nums_factoried.entry(cur).or_default().push(index);
            }
        });

        let mut costs = vec![i32::MAX; nums.len()];
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0))); // (cost, index)
        costs[0] = 0;

        let q_push = |heap: &mut BinaryHeap<Reverse<(i32, usize)>>, costs: &mut Vec<i32>, index: usize, cost: i32| {
            if costs[index] > cost {
                costs[index] = cost;
                heap.push(Reverse((cost, index)));
            }
        };

        while let Some(Reverse((cost, index))) = heap.pop() {
            if costs[index] != cost { continue; }
            if index == nums.len() - 1 { return cost; }
        
            if index > 0 {
                q_push(&mut heap, &mut costs, index - 1, cost + 1);
            }
            if index < nums.len() - 1 {
                q_push(&mut heap, &mut costs, index + 1, cost + 1);
            }

            // nums[index] is a prime number
            if min_prime_factor[nums[index] as usize] == 0 {
                if let Some(vec) = nums_factoried.remove(&(nums[index] as usize)) {
                    vec.iter().for_each(|&i|{
                        q_push(&mut heap, &mut costs, i, cost + 1);
                    });
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_jumps(vec![201,503,363,569,396,270,416,242,288,117,392,696,92,71,218,272,296,241,736,413,389,103,287,781,51,727,315,147]);
        assert_eq!(ret, 18)
    }

    #[test]
    fn unit_test2() {
        let ret = Solution::min_jumps(vec![4,6,5,8]);
        assert_eq!(ret, 3)
    }
}