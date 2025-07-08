use super::Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut ret = 0;
        let mut current_depth = 0;
        let _  = s.chars().for_each(|ch| {
            match ch {
                '(' => current_depth += 1,
                ')' => current_depth -= 1,
                _ => ()
            };

            ret = ret.max(current_depth);

        });
        ret
    }
}
