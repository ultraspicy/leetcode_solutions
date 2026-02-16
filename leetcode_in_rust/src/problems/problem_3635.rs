use std::i32;

use super::Solution;

impl Solution {
    pub fn earliest_finish_time(land_start_time: Vec<i32>, land_duration: Vec<i32>, water_start_time: Vec<i32>, water_duration: Vec<i32>) -> i32 {
        let build_end_time = |start_time: &Vec<i32>, duration: &Vec<i32>| -> Vec<i32> {
            let mut end_time = start_time.iter()
                .zip(duration.iter())
                .map(|(&a, &b)| {
                    a + b
                })
                .collect::<Vec<i32>>();
                end_time.sort_unstable();
                end_time
        };
        let build_result = |end_time: &Vec<i32>, start_times: &Vec<i32>, other_duration: &Vec<i32>|->i32 {
            let end = end_time[0];
            let mut total = i32::MAX;
            start_times.iter().enumerate().for_each(|(i, &start_time)| {
                if start_time <= end {
                    total = total.min(end + other_duration[i]);
                } else {
                    total = total.min(start_time + other_duration[i]);
                }
            });
            total
        };

        let land_end_time = build_end_time(&land_start_time, &land_duration);
        let water_end_time = build_end_time(&water_start_time, &water_duration);
        let opt1 = build_result(&land_end_time, &land_start_time, &water_duration);
        let opt2 = build_result(&water_end_time, &water_start_time, &land_duration);

        opt1.min(opt2)
    }
}
