use super::Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut start: i32 = 0;
        let mut end: i32 = s.len() as i32;

        let mut ret = s.chars()
         . map(|c| {
            if c == 'I' {
                start += 1;
                start - 1
            } else {
                end -= 1;
                end + 1
            }
         })
         .collect::<Vec<_>>();

        ret.push(start);
        ret
    }
}
