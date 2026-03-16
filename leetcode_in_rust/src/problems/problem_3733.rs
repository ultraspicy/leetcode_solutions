use super::Solution;

impl Solution {
    pub fn minimum_time(d: Vec<i32>, r: Vec<i32>) -> i64 {
        let (mut left, mut right) = (0, 100_000_000_000);
        let (d1, d2, r1, r2) = (d[0] as i64, d[1] as i64, r[0] as i64, r[1] as i64);
        while left + 1 < right {
            let mid = (left as i64 + right as i64) / 2;
            if Self::possible(mid, d1, d2, r1, r2) {
                right = mid;
            } else {
                left = mid;
            }
        }
        if Self::possible(left, d1, d2, r1, r2) {
            left 
        } else {
            right 
        }
    }

    fn gcd3733(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { Self::gcd3733(b, a % b) }
    }

    fn possible(time: i64, d1: i64, d2: i64, r1: i64, r2: i64) -> bool {
        // condition1, time - idle >= d_i
        let (idle1, idle2) = (time / r1, time /r2);
        if time - idle1 < d1 {
            return false;
        }
        if time - idle2 < d2 {
            return false;
        }

        // condition2, time - LCA(r1, r2) >= d1+d2
        let gcd = Self::gcd3733(r1, r2);
        //println!("gcd = {}", gcd);
        let lca = (r1 as i64 * r2 as i64) / (gcd as i64);
        if time as i64 - time as i64 / lca < d1 as i64 + d2 as i64 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_possible() {
        let ret = Solution::possible(7, 3, 1,2, 3);
        assert_eq!(true, ret);
    }
}
