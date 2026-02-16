use super::Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let mut rev_iter = s.chars().rev();

        let is_palindrome = s.chars().all(|ch| {
            ch == rev_iter.next().unwrap()
        });

        if is_palindrome {
            1
        } else {
            2
        }

    }
}
