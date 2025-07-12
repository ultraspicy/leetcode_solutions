use super::Solution;

impl Solution {
    pub fn min_moves(sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
        if sx == 0 && sy == 0 {
            if tx == 0 && ty == 0 {
                return 0;
            } else {
                return -1;
            }
        }
        let mut ret = 0;

        let mut tx = tx;
        let mut ty = ty;
        while sx <= tx && sy <= ty {
            if tx < ty {
                if ty > tx * 2 {
                    if ty % 2 != 0 {
                        return -1;
                    } else {
                        ty = ty / 2;
                    }
                } else {
                    ty = ty - tx;
                }
            } else if tx > ty {
                if tx > ty * 2 {
                    if tx % 2 != 0 {
                        return -1;
                    } else {
                        tx /= 2;
                    }
                } else {
                    tx = tx - ty;
                }
            } else {
                if sx == 0 {
                    tx = 0;
                } else if sy == 0 {
                    ty = 0;
                } else {
                    return -1;
                }
            }
            ret += 1;
            if sx == tx && sy == ty {
                return ret;
            }
        }

        -1
    }
}
