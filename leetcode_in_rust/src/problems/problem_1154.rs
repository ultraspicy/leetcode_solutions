use super::Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let days = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let parts: Vec<usize> = date.split("-")
            .map(|s| s.parse().unwrap()).collect();

        let mut ret: i32 = 0;
        if parts[1] >= 3 {
            // leap year
            if (parts[0] % 4 == 0 && parts[0] % 100 != 0) || parts[0] % 400 == 0 {
                ret += 1;
            }
        }

        for i in  0..=parts[1] - 1 {
            ret += days[i]
        }
        ret += parts[2] as i32;

        ret
    }
}
