use leetcode_in_rust::problems::Solution;

fn main() {
    let result = Solution::winning_player_count(5, vec![vec![1, 2], vec![3, 4]]);
    println!("Result of problem 3238: {}", result);
}