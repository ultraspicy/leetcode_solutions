use super::Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut ret: i32 = 0;
        let mut counter = 0;

        s.chars().for_each(|ch| {
            if ch == 'R' {
                counter -= 1;
            } else {
                counter += 1;
            }
            
            if counter == 0 {
                ret += 1;
            }
        });

        ret
    }
}
