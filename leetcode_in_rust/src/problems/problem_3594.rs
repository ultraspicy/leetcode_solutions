use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};
use itertools::Itertools;

use super::Solution;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Solution {
    pub fn min_time_3594(n: i32, k: i32, m: i32, time: Vec<i32>, mul: Vec<f64>) -> f64 {
        if k == 1 && n > 1 {
            return -1.0
        }

        // use last n bit as a bit mask
        let bit_mask: u32 = 0;
        // state is defined as (time to reach this state from init state, bit_mask, boat position, current stage)
        let init_state = (OrderedFloat(0.0), bit_mask, 0 as usize, 0 as usize);
        let final_mask = (0..n).collect::<Vec<_>>()
            .iter()
            .map(|&num| {
                1 << num
            })
            .sum::<u32>();
        println!("final_mask = {}", final_mask);

        let mut heap = BinaryHeap::new();
        heap.push(Reverse(init_state));
        let mut visited = HashSet::new();

        while !heap.is_empty() {
            if let Some(Reverse((accumulted_time, current_mask, position, stage))) = heap.pop() {
                if  current_mask == final_mask {
                    return accumulted_time.0;
                }
                if !visited.insert((current_mask, position, stage)) {
                    continue;
                }
                println!("{:?}", (accumulted_time, current_mask, position, stage));

                // forward path
                if position == 0 {
                    let people_this_trip =  k.min(n - current_mask.count_ones() as i32);
                    let zero_position = (0..n as usize).filter(|i| {
                        current_mask & (1 << i) == 0
                    })
                    .collect::<Vec<_>>();
                    println!("current_mask = {:?}, zero_position {:?}", current_mask,  zero_position);

                    for group_size in 1..=people_this_trip {
                        for group in zero_position.iter().combinations(group_size as usize) {
                            println!("group = {:?}", group);
                            let next_min_time = group.iter().map(|position|{
                                time[**position]
                            })
                            .max().unwrap();
                            let next_min_time = next_min_time as f64 * mul[stage];
                            let new_time = accumulted_time.0 + next_min_time;
                            let new_state = group.iter()
                                .map(|i|{
                                    1 << **i
                                })
                                .sum::<u32>() + current_mask;
                            let new_position = 1;
                            let new_stage = (stage + (next_min_time.floor() as i32 % m) as usize) % mul.len();
                            heap.push(Reverse((OrderedFloat(new_time), new_state, new_position, new_stage)));
                        }
                    }
                } else {
                    let one_position = (0..n as usize).filter(|i| {
                            current_mask & (1 << i) != 0
                        })
                        .collect::<Vec<_>>();
                    println!("current_mask = {:?}, one_position {:?}", current_mask,  one_position);

                    one_position.iter()
                        .for_each(|id| {
                            let next_min_time = time[*id];
                            let next_min_time = next_min_time as f64 * mul[stage];
                            println!("backward {:?}", (next_min_time, id));

                            let new_time = accumulted_time.0 + next_min_time;
                            let new_state = current_mask - (1 << id);
                            //let new_state =  current_mask & !(1 << id);
                            let new_position = 0;
                            let new_stage = (stage + (next_min_time.floor() as i32 % m) as usize) % mul.len();
                            heap.push(Reverse((OrderedFloat(new_time), new_state, new_position, new_stage)));
                        });
                }
            }
        }
        -1 as f64
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test() {
        let ret = Solution::min_time_3594(1, 1, 2, vec![5], vec![1.0,1.3]);
        assert_eq!(ret, 5.0)
    }

    #[test]
    fn unit_test1() {
        let ret = Solution::min_time_3594(3, 2, 3, vec![2, 5, 8], vec![1.0,1.5,0.75]);
        assert_eq!(ret, 14.50)
    }

    #[test]
    fn unit_test2() {
        let ret = Solution::min_time_3594(3, 2, 3, vec![32, 13, 94], vec![1.33,1.41,1.38]);
        assert_eq!(ret, 188.08000)
    }

    #[test]
    fn unit_test3() {
        let ret = Solution::min_time_3594(3, 5, 5, vec![83, 2, 53], vec![1.98,0.75,1.28,0.57,1.88]);
        assert_eq!(ret, 161.14000)
    }

    // #[test]
    // fn unit_test4() {
    //     let ret = Solution::min_time_3594(3, 2, 4, vec![57, 80, 46], vec![1.37,1.81,0.52,1.66]);
    //     assert_eq!(ret, 240.53000)
    // }
}
