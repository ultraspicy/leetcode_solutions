use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn first_unique_freq(nums: Vec<i32>) -> i32 {
        let mut freq_map: HashMap<i32, usize> = HashMap::new();

        nums.iter().for_each(|&num| {
            *freq_map.entry(num).or_default() += 1;
        });

        let mut value_freq: HashMap<usize, usize> = HashMap::new();

        freq_map.values().for_each(|&v| {
            *value_freq.entry(v).or_default() += 1;
        });

        for num in nums {
            if value_freq[&freq_map[&num]] == 1 {
                return num;
            }
        }
        -1
    }

    pub fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
        let mut freq: HashMap<i32, usize> = HashMap::new();

        bulbs.iter().for_each(|&b| {
            *freq.entry(b).or_default() += 1;
        });

        let mut ret = freq.into_iter().filter(|(_, v)| {
            v % 2 == 1
        }).map(|(k, _)| {k}).collect::<Vec<i32>>();

        ret.sort();
        ret
    }
}
