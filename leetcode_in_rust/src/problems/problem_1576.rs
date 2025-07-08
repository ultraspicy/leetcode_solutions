use super::Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();

        for i in 0..chars.len() {
            if chars[i] == '?' {
                for candidate in 'a'..'d' {
                    let prev_ok = (i == 0) || (candidate != chars[i - 1]);
                    let next_ok = (i == chars.len() - 1) || (candidate != chars[i + 1]);

                    if prev_ok && next_ok {
                        chars[i] = candidate;
                    }
                }
            }
        }

        chars.iter().collect::<String>()
    }
}
