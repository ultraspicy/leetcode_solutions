use super::Solution;

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        
        let word_chars = word.chars().collect::<Vec<char>>();
        let abbr_chars = abbr.chars().collect::<Vec<char>>();

        let (mut p1, mut p2) = (0, 0);
        let mut parsed_num= 0;

        while p2 < abbr_chars.len() {
            if abbr_chars[p2].is_ascii_alphabetic() {
                p1 += parsed_num;
                parsed_num = 0;
                if p1 >= word_chars.len() || abbr_chars[p2] != word_chars[p1] {
                    return false;
                }
                p2 += 1;
                p1 += 1;
            } else{
                // no leading zero
                if abbr_chars[p2] == '0' && parsed_num == 0 {
                    return false;
                }
                // parse the number
                parsed_num = parsed_num * 10 + (abbr_chars[p2] as u8 - b'0') as usize;
                p2 += 1;
            }
        }

        p1 += parsed_num;
        p1 == word_chars.len()
    }
}
