use std::borrow::Borrow;
use std::cmp;
use std::collections::{BTreeMap, HashMap};
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::Deref;
use itertools::Itertools;

fn main() {

    // let v: usize = 0;
    // //println!("{}", Solution::punishment_number(37));
    // println!("{}", Solution::min_extra_char("leetscode", vec!["leet","code","leetcode"]));
    // let s = "abcdefg";
    // println!("{}", &s[2..3]);

    let mut vectors = vec![
        vec![1, 2, 5],
        vec![1, 2, 6],
        vec![2, 3, 5],
        vec![2, 3, 6],
    ];

    // Retain vectors where the first two elements are not the same
    let new_vectors: Vec<Vec<i32>> = vectors.iter().map(|x| vec![x[0], x[1] + 1, x[2] + 1]).collect();

    println!("Filtered vectors: {:?}", new_vectors);
    Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1);

}

fn print_vec<T: std::fmt::Display>(vec: &Vec<T>) {
    for e in vec {
        println!("{} \t", e);
    }
}

pub struct Solution {}

#[derive(Default)]
pub struct Trie {
    is_word: bool,
    children: [Box<Option<Trie>>; 26],
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;

        for ch in word.chars() {
            let idx = ch as usize - 'a' as usize;
            node = node.children[idx].get_or_insert_with(|| Trie::new());
        }
        node.is_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = self;
        for ch in word.chars() {
            let idx = ch as usize - 'a' as usize;
            match node.children[idx].deref() {
                None => return false,
                Some(trie) => node = trie,
            }
        }
        node.is_word
    }
}

impl Solution {

    pub fn test () {
        Trie::new();
    }

    pub fn buildTrie(dictionary: Vec<&str>) -> Trie {
        let mut root = Trie::new();
        for word in dictionary {
            root.insert(word);
        }
        root
    }

    pub fn min_extra_char(s: &str, dictionary: Vec<&str>) -> i32 {
        let trie = Solution::buildTrie(dictionary);

        let mut dp =  vec![usize::MAX; s.len() + 1];
        dp[0] = 0;
        for end in 0..s.len() {
            for start in 0..end {
                if trie.contains(s[start..end].as_ref()) {
                    dp[end] = std::cmp::min(dp[end], dp[start]);
                } else {
                    dp[end] = std::cmp::min(dp[end], dp[start] + end - start);
                }
            }
        }
        print_vec(dp.borrow());
        dp[s.len()] as i32
    }


    pub fn punishment_number(n: i32) -> i32 {
        let mut vec = vec![];
        for i in 1..=n {
            let num_in_str = (i * i).to_string();
            let is_punish = Solution::dfs_punishment_number(num_in_str.as_ref(), 0, 0, i);
            if is_punish {
                vec.push(i);
            }
        }
        print_vec(&vec);
        vec.iter().map(|x| x*x).sum()
    }

    fn dfs_punishment_number(number_in_string: &str, sum: i32, start: usize, target: i32) -> bool {
        if sum > target {
            return false;
        }

        let l = number_in_string.len();
        (start..=(l - 1)).any(|i| {
            let parsed_number: Result<i32, _> = number_in_string[start..=i].parse();
            match parsed_number {
                Ok(number) => {
                    if i == (l - 1) {
                        return number + sum == target;
                    }
                    Self::dfs_punishment_number(number_in_string, sum + number, i + 1, target)
                },
                _ => false,
            }
        })
    }

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
        let m: i64 = 1_000_000_000 + 7;
        if nums.len() == 1 {
            let num = nums.get(0).copied().unwrap_or(0) as i64;
            return (Self::pow(num, 3) % m ) as i32;
        }

        let mut nums_mut_ref = nums.clone();
        nums_mut_ref.sort();
        let (mut f_k, mut b_k) = (0, 0);
        for i in 0..nums_mut_ref.len() {
            let num: i64 = nums_mut_ref.get(i).copied().unwrap() as i64;
            f_k = ((Self::pow(num, 3) + Self::pow(num, 2) * b_k as i64) % m  + f_k) % m;
            b_k = ((2 * b_k) % m + num) % m;

        }
        f_k as i32
    }

    pub fn pow(base: i64, exp: i64) -> i64 {
        let m: i64 = 1_000_000_000 + 7;
        let mut ret = 1;
        for _i in 0..exp {
            ret = (ret * base) % m;
        }
        ret
    }

    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let indices : Vec<_> = (1..=n)
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|&num| num.count_ones() == k as u32)
            .collect();

        let mut ret = 0;
        for i in indices {
            ret += nums.get(i).unwrap();
        }
        ret
    }

}