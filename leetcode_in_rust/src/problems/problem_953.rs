use super::Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        words.iter()
            .map(|s| {
                s.chars()
                    .filter_map(|c| order.find(c))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .windows(2)
            .all(|pair| {
                let vec1 = &pair[0];
                let vec2 = &pair[1];
                for i in 0..vec1.len() {
                    if i >= vec2.len() {
                        return false;
                    }
                    if vec1[i] < vec2[i] {
                        return true;
                    } else if vec1[i] == vec2[i] {
                        continue;
                    } else {
                        return false;
                    }
                }
                true
            })
    }
}
