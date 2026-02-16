use super::Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let process = |s: &str| {
            s.chars().fold(Vec::new(), |mut acc, c| {
                    if c == '#' {
                        acc.pop();
                    } else {
                        acc.push(c);
                    }
                    acc
                })
        };

        process(&s) == process(&t)
    }
}
