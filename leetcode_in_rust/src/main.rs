use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Deref;

fn main() {
    // let v = vec![vec![2,4,3,5], vec![5,4,9,3], vec![3,4,2,11]];
    // println!("{}", Solution::max_moves(v));
    let v: usize = 0;
    println!("{}", Solution::sum_of_power(vec![658,489,777,2418,1893,130,2448,178,1128,2149,1059,1495,1166,608,2006,713,1906,2108,680,1348,860,1620,146,2447,1895,1083,1465,2351,1359,1187,906,533,1943,1814,1808,2065,1744,254,1988,1889,1206]));
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

    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in &edges {
            let (u, v) = (edge[0], edge[1]);
            map.entry(u).or_default().push(v);
            map.entry(v).or_default().push(u);
        }

        let mut visited = vec![false; n as usize];
        let mut ret = 0;
        for i in 0..n {
            if !visited.get(i as usize).unwrap() {
                visited[i as usize] = true;
                let mut cc = vec![i];
                // bfs
                let mut queue = VecDeque::new();
                queue.push_back(i);
                while let Some(node) = queue.pop_front() {
                    if let Some(neighbors) = map.get(&node) {
                        for &neigh in neighbors {
                            if !visited.get(neigh as usize).unwrap() {
                                visited[neigh as usize] = true;
                                cc.push(neigh);
                                queue.push_back(neigh);
                            }
                        }
                    }
                }
                if cc.len() == 1 || cc.iter().all(|&x| map[&x].len() == cc.len()) {
                    ret += 1;
                }
            }
        }
        ret
    }

    pub fn sum_of_power(nums: Vec<i32>) -> i32 {
        let m: i64 = 1_000_000_000 + 9;
        if nums.len() == 1 {
            let num = nums.get(0).copied().unwrap_or(0) as i64;
            return (Self::pow(&num, 3) % m ) as i32;
        }

        let mut nums_mut_ref = nums.clone();
        nums_mut_ref.sort();

        let first = nums_mut_ref.get(0).copied().unwrap() as i64;
        let (mut f_k, mut b_k) = (Self::pow(&first, 3), first);
        for i in 1..nums_mut_ref.len() {
            let num: i64 = nums_mut_ref.get(i).copied().unwrap() as i64;
            f_k = ((Self::pow(&num, 3) + Self::pow(&num, 2) * b_k as i64) % m  + f_k) % m;
            b_k = ((2 * b_k) % m + num) % m;

        }
        f_k as i32
    }

    pub fn pow(base: &i64, exp:i32) -> i64 {
        let m: i64 = 1_000_000_000 + 9;
        if exp == 1 {
            return *base as i64
        }

        let mut ret = *base;
        for _i in 1..exp {
            ret = (ret * base) % m;
        }
        ret as i64
    }
}