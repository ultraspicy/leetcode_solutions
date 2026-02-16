use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        visited.insert((x, y));

        path.chars().any(|ch| {
            match ch {
                'N' => y += 1,
                'S' => y -= 1,
                'W' => x -= 1,
                'E' => x += 1,
                _ => ()
            }
            !visited.insert((x, y))
        })
    }
}
