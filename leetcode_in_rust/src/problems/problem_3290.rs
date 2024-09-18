use super::Solution;

impl Solution {
    pub fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
        let a: Vec<i64> = a.into_iter().map(|x| x as i64).collect();
        let b: Vec<i64> = b.into_iter().map(|x| x as i64).collect();
        let mut ret: Vec<i64> = Vec::new();
    
        // Initialize the first row
        ret.push(a[0] * b[0]);
        for i in 1..b.len() {
            ret.push(ret[i - 1].max(a[0] * b[i]));
        }
    
        for i in 1..=3 {
            let prev = ret.clone();
            ret.clear();
            for j in 0..b.len() - i {
                let current = a[i] * b[i + j];
                if j == 0 {
                    ret.push(prev[0] + current);
                } else {
                    ret.push(ret[j - 1].max(prev[j] + current));
                }
            }
        }
    
        *ret.last().unwrap()
    }
}
