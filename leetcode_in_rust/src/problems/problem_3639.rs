use super::Solution;
use std::collections::BTreeSet;

impl Solution {
    pub fn min_time(_s: String, order: Vec<i32>, k: i32) -> i32 {
        if order.len() <= 31 && 2_i32.pow(order.len() as u32) - 1 < k {
            return -1;
        }
        
        let mut ret: i64 = 0;
        let mut visited: BTreeSet<i32> = BTreeSet::new();

        for i in 0..order.len() {
            let left_op = if let Some(&prev) = visited.range(..order[i]).next_back() {
                order[i] - prev
            } else {
                order[i] + 1
            } as i64;

            let right_op = if let Some(&next) = visited.range(order[i]..).next() {
                next - order[i]
            } else {
                order.len() as i32 - order[i]
            } as i64;
            ret += left_op * right_op;
            visited.insert(order[i]);
            if ret >= k as i64{
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_time(String::from("cat"), vec![0, 2, 1], 6);
        assert_eq!(ret, 2);
    }

    #[test]
    fn unit_test2() {
        let ret = Solution::min_time(String::from("yzhh"), vec![2, 1, 0, 3], 10);
        assert_eq!(ret, 3);
    }
}