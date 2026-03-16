use std::vec;

use super::Solution;


struct Sea;

#[allow(dead_code)]
impl Sea {
    fn has_ships(_top_right: Vec<i32>, _bottom_left: Vec<i32>)-> bool {
        false
    }
}
 
#[allow(dead_code)] 
impl Solution {
    fn count_ships(sea: &Sea, top_right: Vec<i32>, bottom_left: Vec<i32>) -> i32 {
        let (x1, y1, x2, y2) = (top_right[0], top_right[1], bottom_left[0], bottom_left[1]);

        if !Sea::has_ships(top_right, bottom_left) {
            return 0;
        }

        let x_range = x1 - x2;
        let y_range = y1 - y2;

        if x_range == 0 && y_range == 0 {
            return 1;
        }

        let mid_x = (x1 + x2) / 2;
        let mid_y = (y1 + y2) / 2;

        if x_range >= y_range {
            let left = Self::count_ships(sea, vec![x1, y1], vec![mid_x+1, y2]);
            let right = Self::count_ships(sea, vec![mid_x, y1], vec![x2, y2]);
            return left + right;
        } else {
            let top = Self::count_ships(sea, vec![x1, y1], vec![x2, mid_y + 1]);
            let bot = Self::count_ships(sea, vec![x1, mid_y], vec![x2, y2]);
            return top + bot;
        }
    }
}
