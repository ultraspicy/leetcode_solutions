use super::Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let odds = nums.iter().map(|&num| {
            if num % 2 == 1 {1} else {0}
        }).sum::<i32>();

        let evens = nums.iter().map(|&num| {
            if num % 2 == 0 {1} else {0}
        }).sum::<i32>();

        if odds.abs_diff(evens) > 1 {
            return -1
        }

        if odds > evens {
            Self::helper(&nums, 0)
        } else if odds == evens {
            Self::helper(&nums, 0).min(Self::helper(&nums, 1))
        } else {
            Self::helper(&nums, 1)
        }
    }

    fn helper (nums: &Vec<i32>, start_odd_index: usize) -> i32 {
        nums.iter().enumerate()
            .filter(|&(_index, &num)|{
                num % 2 == 1
            })
            .map(|(index, _)| {
                index
            })
            .zip((start_odd_index..nums.len()).step_by(2))
            .map(|(a, b)|{
                let abs_diff = a.abs_diff(b);
                abs_diff as i32
            })
            .sum::<i32>()
    }
}
