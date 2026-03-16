use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;
use std::usize;

use super::Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut next = vec![usize::MAX; rains.len()];
        let mut prev = HashMap::new(); //(num, prev index)
        let mut h = BinaryHeap::new();
        let mut ret = Vec::new();
        let mut full_lake = HashSet::new();

        for i in 0..rains.len() {
            let rain = rains[i];

            if prev.contains_key(&rain) {
                next[*prev.get(&rain).unwrap()] = i;
            } 
            prev.insert(rain, i);
        }

        for i in 0..rains.len() {
            let rain = rains[i];

            if rain != 0 {
                if full_lake.insert(rain) {
                    ret.push(-1);
                    h.push((Reverse(next[i]), rain));
                } else {
                    return vec![];
                }    
            } else {
                if let Some((Reverse(_index), lake)) = h.pop() {
                    ret.push(lake);
                    full_lake.remove(&lake);
                } else {
                    ret.push(1);
                }
            }
        } 

        ret
    }
}
