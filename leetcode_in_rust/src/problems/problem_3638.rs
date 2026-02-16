use super::Solution;

impl Solution {
    pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut current_max = -1;

        for i in 0..weight.len() {
            if weight[i] >= current_max {
                current_max = weight[i];
            } else {
                ret += 1;
                current_max = -1
            }
        }

        ret
    }
}
