use super::Solution;

impl Solution {
    pub fn make_good(s: String) -> String {

        s.chars().fold(Vec::new(), |mut acc: Vec<char>, ch: char| {
            if let Some(&last) = acc.last() {
                if ch.to_ascii_lowercase() == last.to_ascii_lowercase() && ch != last {
                    acc.pop();
                } else {
                    acc.push(ch);
                }
            } else {
                acc.push(ch);
            }
            acc
        })
        .iter()
        .collect::<String>()
    }
}
