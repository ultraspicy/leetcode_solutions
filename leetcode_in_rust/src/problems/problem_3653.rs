use super::Solution;

impl Solution {
    pub fn xor_after_queries_3653(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let module: i64 = 1000_000_000 + 7;
        let mut nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        for query in queries {
            let (l, r, k, v) = (query[0], query[1], query[2], query[3]);
            for i in (l..=r).step_by(k as usize) {
                let i = i as usize;
                nums[i] = (nums[i] * v as i64) % module;
            }
        }

        nums.iter().fold(0i64, |acc: i64,  num: &i64| {
            acc ^ num
        }) as i32
    }
}
