use super::Solution;
use itertools::Itertools;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let sliced_name = Self::slice_str(&name);
        let sliced_typed = Self::slice_str(&typed);

        if sliced_name.len() != sliced_typed.len() {
            return false;
        }

        for i in 0..sliced_name.len() {
            let s1 = &sliced_name[i];
            let s2 = &sliced_typed[i];

            if s1.chars().next().unwrap() !=  s2.chars().next().unwrap(){
                return false;
            }
            if s1.len() > s2.len() {
                return false;
            }
        }

        true
    }

    fn slice_str(data: &str) -> Vec<String> {
        data.chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_key, group)| group.collect())
            .collect::<Vec<_>>()
    }
}
