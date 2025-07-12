use super::Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";

        let matrix = words.iter()
            .map(|word| {
                let mut counts = vec![0; alphabet.len()];
                word.chars()
                    .for_each(|c| {         
                        let index = alphabet.find(c).unwrap();
                        counts[index] += 1;
                    });
                counts
            
            })
            .collect::<Vec<_>>();

        (0..alphabet.len())
            .flat_map(|i| {
                let min = matrix.iter()
                    .map(|row| row[i])
                    .min()
                    .unwrap();
                vec![alphabet.chars().nth(i).unwrap().to_string(); min]
            })
            .collect()
    }
}
