use std::cell::RefCell;
use std::rc::{Rc, Weak};
use rand::Rng;

#[derive(Default, Debug, Clone)]
struct Skiplist {
    level: u8,
    dummy_heads: Vec<Rc<RefCell<ListNode>>>,
}

#[derive(Default, Debug, Clone)]
struct ListNode {
    val: i32,

    next: Option<Rc<RefCell<ListNode>>>,
    prev: Option<Weak<RefCell<ListNode>>>,

    below: Option<Rc<RefCell<ListNode>>>,
    above: Option<Weak<RefCell<ListNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl Skiplist {

    pub const LEVEL: u8 = 8;
    const RAND_RANGE: u32 = 1000_000_000;

    pub fn new() -> Self {
        // this is buggy, Rc::clone() will reuse the object, so all dummy heads are actually one!
        // let dummy_heads  = vec![
        //         Rc::new(RefCell::new(ListNode { val: -1, ..Default::default() }));
        //         Self::LEVEL as usize
        //     ];

        let mut dummy_heads  = vec![];
        for i in 0..Self::LEVEL {
            dummy_heads.push(Rc::new(RefCell::new(ListNode { val: (i as i32 *(-1) - 1), ..Default::default() })));
        }

        for i in 0..Self::LEVEL - 1 {
            let cur_node = dummy_heads[i as usize].clone();
            let next = dummy_heads[i as usize + 1].clone();

            cur_node.borrow_mut().below = Some(next.clone());
            next.borrow_mut().above = Some(Rc::downgrade(&cur_node));
        }

        Skiplist {
            level: Self::LEVEL,
            dummy_heads: dummy_heads,
        }
    }

    pub fn search(&self, target: i32) -> bool {
        let mut cur_node = self.dummy_heads[0].clone();

        loop {
            loop {
                let next = cur_node.borrow().next.clone();
                match next {
                    Some(next_node) => {
                        if next_node.borrow().val <= target{
                            cur_node = next_node;
                        } else {
                            let below = cur_node.borrow().below.clone();
                            match below {
                                Some(node_below) => cur_node = node_below,
                                None => break,
                            }
                        }
                    },
                    None => break,
                }
            }
            if cur_node.borrow().val == target { return true; }

            let below = cur_node.borrow().below.clone();
            match below {
                Some(node_below) => cur_node = node_below,
                None => break,
            }
        }
        false
    }

    pub fn add(&self, num: i32) {
        // find the last node that <= num
        // if add, then add this node through all levels
        // if not, go to the next level and rerun the RNG with doubled probability
        let mut rng = rand::rng();
        let mut rng_range = Self::RAND_RANGE / 2_u32.pow(Self::LEVEL as u32);
        let mut cur_node = self.dummy_heads[0].clone();
        let mut cur_level = 0;
        let mut added = false;
        let mut prev_add: Option<Rc<RefCell<ListNode>>> = Option::default();

        loop {
            // horizontally find the last node to operate
            loop {
                let next_option = cur_node.borrow().next.clone();
                match next_option {
                    Some(next_node) => {
                        if next_node.borrow().val <= num {
                            cur_node = next_node;
                        } else {
                            break;
                        }
                    },
                    None => { break; },
                }
            }

            let rng_within_range = rng.random_range(0..Self::RAND_RANGE) < rng_range;
            if rng_within_range {
                added = true;
            }

            if cur_level == Self::LEVEL - 1 || rng_within_range || added {
                // insert a new node
                // deal with horizontal links
                let new_node = Rc::new(RefCell::new(ListNode { val: num, ..Default::default() }));
                let next_option = cur_node.borrow().next.clone();
                if let Some(next_node) = next_option.clone() {
                    next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                }
                new_node.borrow_mut().next = next_option;
                cur_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&cur_node));

                // deal with vertical links
                if let Some(above_node) = prev_add {
                    above_node.borrow_mut().below = Some(new_node.clone());
                    new_node.borrow_mut().above = Some(Rc::downgrade(&above_node));
                }
                prev_add = Some(new_node);
                let below_opt = cur_node.borrow().clone().below.clone();
                match below_opt {
                    Some(below_node) => {
                        cur_node = below_node;
                    },
                    None => {
                        break;
                    }
                }

            } else {
                let below = cur_node.borrow().below.clone().unwrap_or_else(|| panic!("this shall never happen"));
                cur_node = below;
            }
            cur_level += 1;
            rng_range *= 2;
            if cur_level == Self::LEVEL { break; }
        }
    }

    fn erase(&self, num: i32) -> bool {
        let mut cur_node = self.dummy_heads[0].clone();

        loop {
            let next_option = cur_node.borrow().next.clone();
            match next_option {
                Some(next_node) => {
                    if next_node.borrow().val <= num {
                        cur_node = next_node;
                        if cur_node.borrow().val == num {
                            break;
                        }
                    } else {
                        let below_option = cur_node.borrow().below.clone();
                        match below_option {
                            Some(below_node) => { cur_node = below_node },
                            None => {
                                break;
                            }
                        }
                    }
                },
                None => {
                    let below_option = cur_node.borrow().below.clone();
                    match below_option {
                        Some(below_node) => {
                            cur_node = below_node;
                        },
                        None => {
                            break;
                        }
                    }
                }
            }
        }

        if cur_node.borrow().val != num {
            return false;
        }

        loop {
            let below = cur_node.borrow().below.clone();
            if let Some(prev) = cur_node.borrow().clone().prev {
                if let Some(prev_node) = prev.upgrade() {
                    prev_node.borrow_mut().next = cur_node.borrow().clone().next;
                }

                if let Some(next_node) = cur_node.borrow().clone().next {
                    next_node.borrow_mut().prev = Some(prev);
                }
            }
            if let Some(below_node) = below {
                cur_node = below_node;
            } else {

                break;
            }
        }

        true

    }

    pub fn print (&self) {
        println!(" ===========. printing .=============");
        for head in self.dummy_heads.iter() {
            print!("{}", head.borrow().val);
            let mut cur = head.clone();
            while cur.borrow().next.is_some() {
                let next = cur.borrow().next.clone().unwrap();
                print!("->{}",next.borrow().val);
                cur = next;
            }
            println!("");
        }
    }


}

#[cfg(test)]
mod test {
    use std::panic;
    use super::*;

    #[test]
    fn unit_test_new_no_panic() {
        let rst = panic::catch_unwind(|| {
            let _ = Skiplist::new();
        });
        assert!(rst.is_ok(), "Skiplist::new() should not panic")
    }

    #[test]
    fn unit_test_new_nodes_connectivity() {
        let skiplist = Skiplist::new();
        for i in 0..skiplist.level - 1 {
            assert!(skiplist.dummy_heads[i as usize].borrow().below.is_some());
        }
        for i in 1..skiplist.level {
            assert!(skiplist.dummy_heads[i as usize].borrow().above.is_some());
        }
        assert!(skiplist.dummy_heads[skiplist.level as usize - 1].borrow().next.is_none());
    }

    #[test]
    fn unit_test_add_then_search() {
        let skiplist = Skiplist::new();
        skiplist.add(9);
        let rst= skiplist.search(9);
        assert_eq!(rst, true);
    }

     #[test]
    fn unit_test_erase_non_exist() {
        let skiplist = Skiplist::new();
        let rst = skiplist.erase(9);
        assert_eq!(rst, false);
    }

    #[test]
    fn unit_test_erase() {
        let skiplist = Skiplist::new();
        skiplist.add(9);
        let rst = skiplist.erase(9);
        assert_eq!(rst, true);
    }

    #[test]
    fn unit_test_complex() {
        let skiplist = Skiplist::new();
        skiplist.add(1);
        //skiplist.print();
        skiplist.add(2);
        //skiplist.print();
        skiplist.add(3);
        //skiplist.print();
        let rst = skiplist.search(0);
        assert_eq!(rst, false);
        skiplist.add(4);
        let rst = skiplist.search(1);
        assert_eq!(rst, true);
        let rst = skiplist.erase(0);
        assert_eq!(rst, false);
        let rst = skiplist.erase(1);
        assert_eq!(rst, true);
        skiplist.print();
        let rst = skiplist.search(1);
        assert_eq!(rst, false);
    }

}
