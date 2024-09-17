use leetcode_in_rust::problems::Solution;

fn main() {
    let result = Solution::max_score(vec![100000,100000,100000,100000], vec![-100000,-100000,-100000,-100000]);
    println!("Result of problem 3240: {:?}", result);
}