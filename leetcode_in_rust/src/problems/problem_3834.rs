use super::Solution;

impl Solution {
    pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        let mut stack: Vec<i64> = vec![];

        for num in nums.into_iter() {
            let mut cur = num as i64;
            while let Some(top) = stack.last().copied() {
                if top == cur {
                    stack.pop();
                    cur = cur * 2;
                } else {
                    break;
                }
            }
            stack.push(cur);
        }

        stack
    }
}
