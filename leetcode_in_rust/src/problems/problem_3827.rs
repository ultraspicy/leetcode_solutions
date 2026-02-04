use super::Solution;

impl Solution {

    pub fn count_monobit(n: i32) -> i32 {
        
        if n == 0 {
            return 1;
        } else if n == 1 {
            return 2;
        } else {
            let over_three: i32 = (3..=n).map(|number| {
                let mut t = number;
                while t > 0 {
                    if t % 2 == 0 {
                        return 0;
                    } 
                    t = t / 2; 
                }
                1
            }).sum();
            
            return over_three + 2
        }
    }
}
