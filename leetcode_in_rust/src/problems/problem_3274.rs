use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3274
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        (coordinate1.chars().nth(0).unwrap() as u8 - b'a' + coordinate1.chars().nth(1).unwrap() as u8 - b'0'
        - (coordinate2.chars().nth(0).unwrap() as u8 - b'a' + coordinate2.chars().nth(1).unwrap() as u8 - b'0')) % 2 == 0
    }
}
