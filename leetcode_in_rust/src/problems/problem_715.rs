use std::collections::BTreeMap;

struct RangeModule {
    left_to_right: BTreeMap<i32, i32>,
    right_to_left: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {

    fn new() -> Self {
        RangeModule { left_to_right: BTreeMap::default(), right_to_left: BTreeMap::default() }
    }
    
    fn add_range(&mut self, left: i32, right: i32) {
        let (mut ll, mut rr) = (-1, -1);
        match self.left_to_right.range(..left).next_back() {
            Some((&a, &b)) => {
                ll = if b >= left { a } else { left }; 
            },
            None => {
                ll = left;
            },
        }

        match self.right_to_left.range(right..).next() {
            Some((&d, &c)) => {
                rr = if c <= right { d } else { right };
            },
            None => {
                rr = right;
            },
        }

        // collect consumes the iterator, ending the borrow
        let to_remove = self.left_to_right
            .range(ll..)
            .filter(|&(&k, &v)| {k <= rr})
            .map(|(&k, &v)| {(k,v)})
            .collect::<Vec<(i32, i32)>>();
        for (k, v) in to_remove {
            self.left_to_right.remove(&k);
            self.right_to_left.remove(&v);
        }
       
        self.left_to_right.insert(ll, rr);
        self.right_to_left.insert(rr, ll);
    }
    
    fn query_range(&self, left: i32, right: i32) -> bool {
        match self.left_to_right.range(..=left).next_back() {
            Some((&l, &r)) => {
                r >= right
            },
            None => {
                false
            },
        }
    }
    
    fn remove_range(&mut self, left: i32, right: i32) {
        let to_remove = self.left_to_right
            .range(left..)
            .filter(|&(&k, &v)| {k >= left && v <= right})
            .map(|(&k, &v)| {(k,v)})
            .collect::<Vec<(i32, i32)>>();
        for (k, v) in to_remove {
            self.left_to_right.remove(&k);
            self.right_to_left.remove(&v);
        }

        let mut to_add = vec![];
        let mut to_remove = vec![];
        match self.right_to_left.range(left..).next() {
            Some((&b, &a)) => {
                if a < left {
                    // self.left_to_right.insert(a, left);
                    // println!("a = {}, b = {}", a, b);
                    // self.right_to_left.insert(left, a);
                    to_add.push((a, left));
                    to_remove.push((a, b));
                }
                // else this interval removed already
            },
            None => {
                // still fine to remove from left 
            },
        }

        match self.left_to_right.range(..right).next_back() {
            Some((&c, &d)) => {
                if d > right {
                    // self.right_to_left.insert(d, right);
                    // self.left_to_right.insert(right, d);
                    to_add.push((right, d));
                    to_remove.push((c, d));
                }
                // else this interval removed already
            },
            None => {
                // still fine to remove untill right
            },
        }

        for &(k, v) in to_remove.iter() {
            self.left_to_right.remove(&k);
            self.right_to_left.remove(&v);
        }

        for &(k, v) in to_add.iter() {
            self.left_to_right.insert(k, v);
            self.right_to_left.insert(v, k);
        }
    }

    fn print(&self) {
        println!("==============================================");
        for (&k, &v) in self.left_to_right.iter() {
            print!("({}, {}) \t", k, v);
        }
         println!("")
    }
}

#[cfg(test)]
mod test {
    use crate::problems::problem_715::RangeModule;

    #[test]
    fn unit_test() {
        let mut range_module = RangeModule::new();
        range_module.add_range(10, 20);
       
        range_module.remove_range(14, 16);
        range_module.print();
        range_module.query_range(10, 14);
        range_module.query_range(13, 15);
        range_module.query_range(16, 17);
    }
}