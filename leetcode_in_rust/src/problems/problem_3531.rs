use std::collections::HashMap;

use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3531
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut y_min: HashMap<i32, i32> = HashMap::new(); // given x as key, what is the smallest y
        let mut y_max: HashMap<i32, i32> = HashMap::new();
        let mut x_min: HashMap<i32, i32> = HashMap::new(); // given y as key, what is the smallest x
        let mut x_max: HashMap<i32, i32> = HashMap::new();

        buildings.iter().for_each(|building| {
            let (x, y) = (building[0], building[1]);
            y_min.entry(x).and_modify(|prev_min| *prev_min = (*prev_min).min(y)).or_insert(y);
            y_max.entry(x).and_modify(|prev_max| *prev_max = (*prev_max).max(y)).or_insert(y);
            x_min.entry(y).and_modify(|prev_min| *prev_min = (*prev_min).min(x)).or_insert(x);
            x_max.entry(y).and_modify(|prev_max| *prev_max = (*prev_max).max(x)).or_insert(x);
        });

        buildings.iter().map(|building| {
            let (x, y) = (building[0], building[1]);
            if *y_min.get(&x).expect("key guranteed to exist") == y || *y_max.get(&x).expect("key guranteed to exist") == y
                || *x_min.get(&y).expect("key guranteed to exist") == x || *x_max.get(&y).expect("key guranteed to exist") == x{
                    return 0;
            }
            1
        }).sum::<i32>()
    }
}
