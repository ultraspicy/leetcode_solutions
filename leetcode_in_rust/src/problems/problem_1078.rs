use super::Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        text.split_ascii_whitespace()
            .collect::<Vec<_>>()
            .windows(3)
            .filter(|&tuple| {
                tuple[0] == first && tuple[1] == second
            })
            .map(|tuple| {
                tuple[2].to_string()
            })
            .collect::<Vec<String>>()
    }
}
