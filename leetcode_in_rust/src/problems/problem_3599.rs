use super::Solution;

impl Solution {
    pub fn min_xor(nums: Vec<i32>, k: i32) -> i32 {
        let mut pref_xor = vec![0; nums.len() + 1]; 
        let mut running_xor: i32 = 0;
        // prefix xor, index i means xor (0..=i) 
        (1..=nums.len()).into_iter().for_each(|i|{
            running_xor = running_xor ^ (nums[i - 1] as i32);
            pref_xor[i] = running_xor;
        });

        if k == 1 {
            return running_xor as i32;
        }

        let dp_cur: &mut Vec<i32> = &mut pref_xor.clone();
    
        (2..=k).into_iter().for_each(|num_of_partition| { // number of partition
            let dp_next: &mut Vec<i32> = &mut vec![i32::MAX; nums.len() + 1];
            (num_of_partition..=nums.len() as i32).into_iter().for_each(|i| { // first (exclusive) i element
                ((num_of_partition - 1)..=(i - 1)).into_iter().for_each(|j|{
                    dp_next[i as usize] = dp_next[i as usize].min(
                        dp_cur[j as usize].max(pref_xor[i as usize] ^ pref_xor[j as usize])
                    )
                });
            });
            std::mem::swap(dp_cur, dp_next);
        });

        dp_cur[nums.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test (){
        let ret = Solution::min_xor(vec![1,2,3], 2);
        assert_eq!(1, ret)
    }

    #[test]
    fn unit_test2 (){
        let ret = Solution::min_xor(vec![2,3,3,2], 3);
        assert_eq!(2, ret)
    }

    #[test]
    fn unit_test3 (){
        let ret = Solution::min_xor(vec![1,1,2,3,1], 2);
        assert_eq!(0, ret)
    }
}
