use super::Solution;

impl Solution {
    pub fn bowl_subarrays(nums: Vec<i32>) -> i64 {
        let mut stack: Vec<i32> = Vec::new();
        let mut ret: i64 = 0;
        for num in nums {
            while let Some(&top) = stack.last() {
                if top <= num {
                    stack.pop();
                    if stack.len() >= 1 {
                        ret += 1;
                    }

                } else {
                    break;
                }
            }
            stack.push(num);
        }
        ret
    }
}
