use super::Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut iter_alpha = s
            .chars()
            .rev()
            .filter(|char| char.is_ascii_alphabetic());
        
        s.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    iter_alpha.next().unwrap()
                } else {
                    c
                }
            })
            .collect()
    }
}
