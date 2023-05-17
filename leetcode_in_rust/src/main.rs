use std::collections::HashMap;

fn main() {
    // let v = vec![vec![2,4,3,5], vec![5,4,9,3], vec![3,4,2,11]];
    // println!("{}", Solution::max_moves(v));
    let v: usize = 0;
    println!("{}", v as i32 - 1);
}

fn print_vec(vec: &Vec<i32>) {
    for e in vec {
        println!("{} \t", e);
    }
}

pub struct Solution {}

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        map.insert(1, 0);
        let (mut player, mut hop)= (1, k);
        loop {
            player = (player + hop) % n;
            if player == 0 {
                player = n;
            }
            if let Some(&_v) = map.get(&player) {
                break;
            }
            map.insert(player, 0);
            hop = hop + k;
        }
        let losers = (1..=n).filter(|&f| !map.contains_key(&f)).collect();

        return losers;
    }

    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut a = 0;
        for num in derived {
            a = a ^ num;
        }
        return a == 0
    }

    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut cache: Vec<Vec<usize>> = vec![vec![0; m]; n];
        let mut ret: i32 = 0;
        for i in 0..m {
            ret = ret.max(Self::dfs(&grid, i, 0, 0, &mut cache) as i32);
        }
        return ret;
    }

    pub fn dfs(
        grid: &Vec<Vec<i32>>,
        last_row: usize,
        last_col: usize,
        steps: usize,
        cache: &mut Vec<Vec<usize>>,
    ) -> usize {
        let (m, n) = (grid.len(), grid[0].len());
        if last_col == n - 1 {
            return n - 1;
        }
        if cache[last_row][last_col] != 0 {
            return cache[last_row][last_col];
        }

        let (mut ret, val) = (0, grid[last_row][last_col]);
        if last_row as i32 - 1 >= 0 && grid[last_row - 1][last_col + 1] > val {
            ret = ret.max(Self::dfs(grid, last_row - 1, last_col + 1, steps + 1, cache));
        }
        if grid[last_row][last_col + 1] > val {
            ret = ret.max(Self::dfs(grid, last_row, last_col + 1, steps + 1, cache));
        }
        if (last_row + 1) < m && grid[last_row + 1][last_col + 1] > val {
            ret = ret.max(Self::dfs(grid, last_row + 1, last_col + 1, steps + 1, cache));
        }
        if ret == 0 {
            ret = steps;
        }
        cache[last_row][last_col] = ret;
        return ret;
    }
}