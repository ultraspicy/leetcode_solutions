use std::{collections::{HashMap, HashSet}, hash::Hash};

use super::Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let sources: HashSet<_> = paths.iter().map(|p| &p[0]).collect();
        paths.iter().find_map(|p| (!sources.contains(&p[1])).then(|| p[1].clone())).unwrap()
    }
}
