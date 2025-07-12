use super::Solution;

impl Solution {
    pub fn min_cost(m: i32, n: i32, wait_cost: Vec<Vec<i32>>) -> i64 {
        let mut matrix: Vec<Vec<(i64, i64)>> = vec![vec![(-1, -1); n as usize]; m as usize];
        // paything the enter cost of (0 + 1)*(0 + 1)
        // tuple (waiting, entering)
        matrix[0][0] = (1, 1); 

        for i in 1..n as usize {
            let enter_from_left = matrix[0][i - 1].0 + (i + 1) as i64;
            let waiting = enter_from_left + wait_cost[0][i] as i64;
            matrix[0][i] = (waiting, enter_from_left);
        }

        for j in 1..m as usize {
            let enter_from_top = matrix[j - 1][0].0 + (j + 1) as i64;
            let waiting = enter_from_top + wait_cost[j][0] as i64;
            matrix[j][0] = (waiting, enter_from_top);
        }

        for i in 1..m as usize {
            for j in 1..n as usize {
                let entering_accumuted = matrix[i - 1][j].0.min(matrix[i][j - 1].0) + (i as i64 + 1) * (j as i64 + 1);
                let waiting_accumuted = entering_accumuted + wait_cost[i][j] as i64;
                matrix[i][j] = (waiting_accumuted, entering_accumuted);
            }
        }

        matrix[m as usize - 1][n as usize - 1].1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_cost(2, 3, vec![vec![6,1,4], vec![3,2,5]]);

        assert_eq!(ret, 16);
    }
}
