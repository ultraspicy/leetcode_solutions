use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 1975
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let n = matrix.len();
        let mut ret: i64 = 0;
        let mut is_even_negative = true;
        let mut min_abs = i32::MIN + 1;
        let mut has_zero = false;

        for i in 0..n {
            for j in 0..n {
                let num = matrix[i][j];
                ret += num.abs() as i64;
                if num < 0 {
                    is_even_negative = !is_even_negative;
                }
                if num == 0 {
                    has_zero = true;
                }
                if num.abs() < min_abs.abs() {
                    min_abs = num;
                }
            }
        }

        if !is_even_negative && !has_zero {
            ret + -2 * (min_abs.abs() as i64)
        } else {
            ret
        }
    }
}
