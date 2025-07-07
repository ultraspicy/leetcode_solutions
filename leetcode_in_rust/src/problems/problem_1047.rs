use super::Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        s.chars().fold(Vec::new(), |mut acc: Vec<char>, ch|{
            match acc.last() {
                Some(&c) => {
                    if c == ch {
                        acc.pop();
                    } else {
                        acc.push(ch);
                    }
                    acc
                },
                None => { acc.push(ch); acc }
            }
        }).into_iter().collect()
    }
}
