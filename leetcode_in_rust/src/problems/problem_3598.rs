use std::vec;

use super::Solution;

impl Solution {
    pub fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
        if words.len() <= 2 {
            return vec![0; words.len()];
        }

        let longest_prefix  = &mut (0, 0, 0);
        let second_longest = &mut (0, 0, 0);
        let third_longest = &mut (0, 0, 0);

        for i in 0..words.len() - 1 {
            let prefix_of_next = Self::common_prefix_len(&words[i], &words[i + 1]);
            Self::cascade(
                i, 
                i + 1, 
                prefix_of_next, 
                longest_prefix, 
                second_longest, 
                third_longest,
            );
        }

        println!("{:?}", longest_prefix); 
        println!("{:?}", second_longest);
        println!("{:?}", third_longest);

        let mut ret = vec![];
        for i in 0..words.len() {
            let mut prefix_with_jump = 0;
            let mut prefix_without_jump = 0;
            if longest_prefix.1 != i && longest_prefix.2 != i {
                prefix_without_jump = longest_prefix.0 
            } else if second_longest.1 != i && second_longest.2 != i {
                prefix_without_jump = second_longest.0 
            } else {
                prefix_without_jump = third_longest.0 
            }
            if i != 0  && i != words.len() - 1{
                prefix_with_jump =  Self::common_prefix_len(&words[i - 1], &words[i + 1]);
            }
            
            ret.push(prefix_with_jump.max(prefix_without_jump) as i32);
        }

        ret
    }

    fn common_prefix_len(s1: &str, s2: &str) -> usize {
        s1.chars().zip(s2.chars())
            .take_while(|&(c1, c2)| c1 == c2)
            .count() 
    }

    fn cascade (
        start_index: usize, 
        end_index: usize,
        prefix_len: usize,
        longest_prefix: &mut (usize, usize, usize), 
        second_longest: &mut (usize, usize, usize),
        third_longest: &mut (usize, usize, usize),
    ) {
        if prefix_len >= longest_prefix.0 {
            *third_longest = *second_longest;
            *second_longest = *longest_prefix;
            *longest_prefix = (prefix_len, start_index, end_index);
        } else if prefix_len >= second_longest.0 {
            *third_longest = *second_longest;
            *second_longest = (prefix_len, start_index, end_index);
        } else if prefix_len >= third_longest.0 {
            *third_longest = (prefix_len, start_index, end_index);
        }
    } 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::longest_common_prefix(vec!["jump".to_string(),"run".to_string(),"run".to_string(),"jump".to_string(),"run".to_string()]);
        assert_eq!(vec![3,0,0,3,3], ret);
    }
}