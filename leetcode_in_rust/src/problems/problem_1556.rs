use super::Solution;

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let s = n.to_string();
        let len = s.len();

        match n {
            0..1000 => s,
            1000..1_000_000 => format!("{}.{}", &s[..len-3], &s[len-3..]),
            1_000_000..1_000_000_000 => format!("{}.{}.{}", &s[..len-6], &s[len-6..len-3], &s[len-3..]),
            _ => format!("{}.{}.{}.{}", &s[..len-9], &s[len-9..len-6], &s[len-6..len-3], &s[len-3..])
        }
    }
}
