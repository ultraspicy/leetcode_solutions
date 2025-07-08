use super::Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let num_of_whitespace: usize = text.chars().map(|ch| {
            match ch {
                ' ' => 1,
                _ => 0
            }
        })
        .sum();
        let words: Vec<&str> = text.split_ascii_whitespace().collect::<Vec<_>>();
        if words.len() == 1 {
            return format!(
                "{}{}",
                words[0],
                " ".repeat(text.len() - words[0].len())
            )
        }

        let betwee_word_whitespace = num_of_whitespace / (words.len() - 1);
        let trailing_whitespace = num_of_whitespace % (words.len() - 1);
        format!("{}{}", words.join(&" ".repeat(betwee_word_whitespace)), " ".repeat(trailing_whitespace))
    }
}
