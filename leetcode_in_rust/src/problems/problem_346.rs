#[derive(Debug, Default)]
#[allow(dead_code)]
struct MovingAverage {
    size: i32,

    sum: f64,
    cur_size: i32,
    moving_vec: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MovingAverage {

    fn new(size: i32) -> Self {
        MovingAverage {
            size: size,
            ..Default::default()
        }
    }
    
    fn next(&mut self, val: i32) -> f64 {
        self.sum += val as f64;
        self.moving_vec.push(val);
        if self.cur_size < self.size {
            self.cur_size += 1;
        } else {
            self.sum -= self.moving_vec[self.moving_vec.len() - self.size as usize - 1] as f64;

        }

        self.sum / (self.cur_size as f64)
    }
}

