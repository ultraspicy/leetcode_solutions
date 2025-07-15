use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn process_str_3614(s: String, k: i64) -> char {
        let mut map: HashMap<usize, char> = HashMap::new();

        let mut len = 0;
        for (_, ch) in s.char_indices() {
            if ch.is_ascii_alphabetic() {
                map.insert(len, ch);
                len += 1;
            } else if ch == '*' && len > 0 {
                len -= 1;
            } else if ch == '#' {
                len *= 2;
            }
        }

        if k as usize >= len {
            return '.'
        }

        let mut k: usize = k as usize;
        for ch in s.chars().rev() {
            println!("(len, k) = {:?}", (len, k));
            if ch == '#' {
                len = len / 2;
                if k >= len {
                    k = k - len;
                }
            } else if ch == '*' {
                len = len + 1;
            } else if ch == '%' {
                k = len - 1 - k;
            } else {
                len = len - 1;
                if len == k {
                    return ch
                }
            }
        }
        'x'
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test () {
        let ret = Solution::process_str_3614(String::from("a#b%*"), 1);
        assert_eq!(ret, 'a');
    }

    #[test]
    fn unit_test2 () {
        let ret = Solution::process_str_3614(String::from("cd%#*#"), 3);
        assert_eq!(ret, 'd');
    }

    #[test]
    fn unit_test3 () {
        let ret = Solution::process_str_3614(String::from("qud#y"), 2);
        assert_eq!(ret, 'd');
    }

    #[test]
    fn unit_test4 () {
        let ret = Solution::process_str_3614(String::from("%#*gm#xib"), 2);
        assert_eq!(ret, 'g');
    }
}
