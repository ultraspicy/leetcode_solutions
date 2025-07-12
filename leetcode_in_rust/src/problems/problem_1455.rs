use super::Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let words = sentence.split_ascii_whitespace().collect::<Vec<_>>();

        words.iter()
            .enumerate()
            .find(|(_, &s)| { Self::is_prefix(s, &search_word)})
            .map(|(i, _)| { (i + 1) as i32 })
            .unwrap_or(-1)
    }

    fn is_prefix(s: &str, prefix: &str) -> bool {
        if s.len() < prefix.len() {
            return false
        }
        s.chars().zip(prefix.chars())
            .all(|(c1, c2)| c1 == c2)
    }
}
