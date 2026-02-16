use super::Solution;

impl Solution {
    pub fn process_str(s: String) -> String {

        s.chars().fold(Vec::new(), |mut acc: Vec<char>, ch| {
            if ch == '*' && acc.len() > 0 {
                acc.pop();
            } else if ch == '#' {
                acc.append(&mut acc.clone());
            } else if ch == '%' {
                acc = acc.into_iter().rev().collect::<Vec<_>>();
            } else if ch.is_ascii_alphabetic() {
                acc.push(ch);
            }
            acc
        }).iter().collect::<String>()
    }

}
