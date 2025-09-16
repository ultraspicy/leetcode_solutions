use super::Solution;

impl Solution {

    pub fn min_operations(s: String) -> i32 {
        let mut ret = 0;
        s.chars().for_each(|c| {
            ret = ret.max(Self::char_to_index(c));
        });
        ret
    }

    fn char_to_index(c: char) -> i32 {
        let zero_index = c as u8 - b'a';
        (26 - zero_index as i32) % 26
    }
}
