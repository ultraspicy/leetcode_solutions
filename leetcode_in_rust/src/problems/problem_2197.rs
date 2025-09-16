use std::vec;

use super::Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![];

        for num in nums {
            let mut tmp = num;
            while let Some(&last) = ret.last() {
                let gcd = Self::greatest_common_divisor(last, tmp);
                if gcd == 1 {
                    // if coprime, append and break the current loop
                    ret.push(tmp);
                    break;
                } else {
                    // if not, pop the last and work with new LCM
                    tmp = ret.pop().unwrap() / gcd * tmp;
                    //println!(" tmp = {:?}, ret = {:?}", tmp, ret);
                }                
            }
        
            if ret.is_empty() {
                ret.push(tmp);
            }
            //println!(" ret  = {:?}", ret);
        }
        ret
    }

    fn greatest_common_divisor (a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::greatest_common_divisor(b, a % b) }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        let a = Solution::greatest_common_divisor(6, 4);
        assert_eq!(a, 2);

        let a = Solution::greatest_common_divisor(12, 13);
        assert_eq!(a, 1);

        let a = Solution::greatest_common_divisor(12, 16);
        assert_eq!(a, 4);
    }

    #[test]
    fn test_replace_non_coprimes() {
        let a = Solution::replace_non_coprimes(vec![6,4,3,2,7,6,2]);
        println!(" a = {:?}", a);
        assert_eq!(a, vec![12,7,6]);
    }
}