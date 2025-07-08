use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut position_map: HashMap<char, i32> = HashMap::new();
        let mut ret: i32 = -1;
        s.chars().enumerate()
            . for_each(|(index, ch)| {
            if let Some(&position) = position_map.get(&ch) {
                ret = ret.max(index as i32 - position - 1);
            } else {
                position_map.insert(ch, index as i32);
            }
        });
        ret as i32
    }
}
