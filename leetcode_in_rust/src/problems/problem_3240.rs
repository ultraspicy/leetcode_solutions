use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3240
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut ret: i32 = 0; 
        // 4 way symmetry
        for i in 0..m / 2 {
            for j in 0.. n /2 {
                let tmp = grid[i][j] + grid[m - 1 - i][j] + grid[i][n - 1 - j] + grid[m - 1 - i][n - 1 - j];
                ret += tmp.min(4 - tmp);
            }
        }

        // two way symmtry
        let mut one_one_pair: i32 = 0;
        let mut one_zero_pair: i32 = 0;
        if m % 2 == 1 {
            let mid_row =&grid[m/2];
            for i in 0..n / 2 {
                if mid_row[i] == 1 && mid_row[n  - 1 - i] == 1 {
                    one_one_pair += 1;
                }
                if (mid_row[i] == 0 && mid_row[n - 1 - i] == 1) || (mid_row[i] == 1 && mid_row[n - 1 - i] == 0) {
                    one_zero_pair += 1;
                }
            }
        }
        if n % 2 == 1 {
            let mid_col: Vec<i32> = grid.iter().map(|row| row[n / 2]).collect(); 
            for i in 0..m / 2 {
                if mid_col[i] == 1 && mid_col[m - 1 - i] == 1 {
                    one_one_pair += 1;
                }
                if (mid_col[i] == 0 && mid_col[m - 1 - i] == 1) || (mid_col[i] == 1 && mid_col[m - 1 - i] == 0) {
                    one_zero_pair += 1;
                }
            }
        }
    
        if m % 2 == 1 && n % 2 == 1 && grid[m/2][n/2] == 1{
            ret += 1;
        }
    
        println!("one_one_pair = {}, one_zero_pair = {}, ret = {}", one_one_pair, one_zero_pair, ret);
    
        if one_one_pair % 2 == 0 {
            ret + one_zero_pair
        } else {
            if one_zero_pair > 0 {
                ret + one_zero_pair
            } else {
                ret + 2
            }
        }
       
    }
}
