use super::Solution;

impl Solution {
    // use type coerce to make function more generic
    // not use `count_row_flip(row: Vec<i32>)` cuz it will take the ownership
    // and Partial moves are disallowed, so `count_row_flip(grid[i])` is disallowed
    fn count_flip(row: &[i32]) -> i32 {
        row.iter().zip(row.iter().rev()).take(row.len() / 2).filter(|(&a, &b)| a != b).count() as i32
    }

    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut row_flip = 0;
        for i in 0..m {
            row_flip += Solution::count_flip(&grid[i]); 
        }

        let mut col_flip = 0;
        for j in 0..n {
            let col: Vec<i32> = grid.iter().map(|row| row[j]).collect(); 
            col_flip += Solution::count_flip(&col);
        }
        
        row_flip.min(col_flip)
    }
}
    // TODO: Implement the method for problem 3239
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }


