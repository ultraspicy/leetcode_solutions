use std::collections::{HashSet, VecDeque};

use super::Solution;

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut visited = HashSet::new();
        let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if visited.insert((i, j)) {
                    let mut q = VecDeque::new();
                    let mut area = 0;
                    q.push_back((i, j));
                    while let Some((x, y)) = q.pop_front() {
                        area += grid[x as usize][y as usize];
                        for direction in directions.iter() {
                            let new_x = x as i32 + direction.0;
                            let new_y = y as i32 + direction.1;
                            if new_x >= 0 && new_x < m && new_y >= 0 && new_y < n && visited.insert((new_x, new_y)) && grid[new_x as usize][new_y as usize] != 0 {
                                q.push_back((new_x, new_y));
                            }
                        }
                    }
                    if area % k == 0 {
                        ret += 1;
                    }
                }
            } 
        }

        ret
    }
}
