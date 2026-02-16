use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, i64> = HashMap::new();
        let m: i64 = 1000_000_000 + 7;


        points.iter().for_each(|pair| {
            let (_, y) = (pair[0], pair[1]);
            *map.entry(y).or_insert(0) += 1;
        });

        let row_combinations = map.values().collect::<Vec<_>>()
            .iter()
            .map(|&&occ| occ * (occ - 1) / 2)
            .filter(|&n| n > 0)
            .collect::<Vec<_>>();

        let a: i64 = row_combinations.iter().sum::<i64>();
        let b: i64 = row_combinations.iter().map(|num|{
            num * num % m
        }).sum::<i64>();

        let mut ans = a * a - b;
        ans /= 2;
        ans %= m;
        ans as i32
    }
}
