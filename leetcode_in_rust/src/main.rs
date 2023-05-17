use std::collections::HashMap;

fn main() {
    let ret = Solution::circular_game_losers(5, 2);
    print_vec(ret.as_ref());
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
}