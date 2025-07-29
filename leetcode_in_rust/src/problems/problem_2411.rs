use super::Solution;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut max_value = nums.iter().max().copied().unwrap();
        let mut nums_mut = nums.clone();
        let mut ret = vec![1; n];

        while max_value != 0 {
            let mut last_seen_one = n;
            for i in 0..n {
                // pos = n - 1 - i
                if nums_mut[n - 1 - i] % 2 == 1 {
                    last_seen_one = n - 1 - i;
                } else {
                    if last_seen_one != n {
                        let len = last_seen_one - (n - 1 -i) + 1;
                        ret[n - i - 1] = ret[n - 1 - i].max(len);
                    }
                }
                nums_mut[n - 1 - i] = nums_mut[n - 1 - i] / 2;
            }
            max_value = max_value / 2;
        }

        ret.iter().map(|&num| { num as i32}).collect()
    }
}
