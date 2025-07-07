use super::Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
       let mut freq = vec![0; 26];

       text.chars()
        .for_each(|ch| {
            freq[ch as usize - 'a' as usize] += 1;
        });

       let vec = vec![freq[0], freq[1], freq[11]/2, freq[13], freq[14]/2];
       
       vec.into_iter().min().unwrap() 
    }
}
