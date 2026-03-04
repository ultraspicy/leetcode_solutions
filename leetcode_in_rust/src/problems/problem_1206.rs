use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::vec;
use rand::Rng;

type NodeRef<T> = Rc<RefCell<ListNode<T>>>;
type WeakRef<T> = Weak<RefCell<ListNode<T>>>;

#[derive(Default, Debug, Clone)]
struct Skiplist {
    heads: Vec<Option<NodeRef<i32>>>
}

#[derive(Default, Debug, Clone)]
struct ListNode<T: Ord> {
    val: T,
    next: Option<NodeRef<T>>,
    below: Option<NodeRef<T>>,
    above: Option<WeakRef<T>>,
}

impl<T: Ord> ListNode<T> {
    fn new_rc(val: T) -> NodeRef<T> {
        Rc::new(RefCell::new(Self::new(val)))
    }
    
    fn new(val: T) -> ListNode<T> {
        ListNode {
            val: val,
            next: Option::default(),
            below: Option::default(),
            above: Option::default(),
        }
    }
}

#[allow(dead_code)]
impl Skiplist {

    fn new() -> Self {
        // for LC, we make it a level of 4
        let mut heads = vec![];
        (0..4).for_each(|_lvl| {
            heads.push(Some(ListNode::new_rc(-1)));
        });
        Skiplist { heads:heads }
    }
    
    fn search(&self, target: i32) -> bool {
        let candidate = self.search_internal(target);
        let ret = candidate.borrow().val == target;
        ret
    }
    
    fn add(&mut self, num: i32) {
        // go to the target level first
        let mut cursor = self.heads[0].clone().unwrap();
        let mut r = rand::rng();
        for i in 0..4 {
            if r.random::<bool>() {
                let next_rc = cursor.borrow().below.clone().unwrap();
                cursor = next_rc;
            }
        }
        // search the last item <= num, then add itself and all node below
    }
    
    fn erase(&mut self, num: i32) -> bool {
        // search_internal get the node
        true
    }

    fn search_internal(&self, target: i32) -> NodeRef<i32> {
        let mut cursor = self.heads[0].clone();
        (0..4).for_each(|_lvl| {
            while let Some(next_rc) = cursor.clone() {
                let val = next_rc.borrow().val;
                if val <= target {
                    cursor = Some(next_rc.clone());
                } else {
                    break;
                }
            }
            
            if let Some(below_rc) = cursor.clone() {
                cursor = Some(below_rc)
            } 
        });

        cursor.expect("unwrapping null, this should not happen")
    }
}

// #[cfg(test)]
// mod test {
//     use std::panic;
//     use super::*;

//     #[test]
//     fn unit_test_new_no_panic() {
//         let rst = panic::catch_unwind(|| {
//             let _ = Skiplist::new();
//         });
//         assert!(rst.is_ok(), "Skiplist::new() should not panic")
//     }

//     #[test]
//     fn unit_test_new_nodes_connectivity() {
//         let skiplist = Skiplist::new();
//         for i in 0..skiplist.level - 1 {
//             assert!(skiplist.dummy_heads[i as usize].borrow().below.is_some());
//         }
//         for i in 1..skiplist.level {
//             assert!(skiplist.dummy_heads[i as usize].borrow().above.is_some());
//         }
//         assert!(skiplist.dummy_heads[skiplist.level as usize - 1].borrow().next.is_none());
//     }

//     #[test]
//     fn unit_test_add_then_search() {
//         let skiplist = Skiplist::new();
//         skiplist.add(9);
//         let rst= skiplist.search(9);
//         assert_eq!(rst, true);
//     }

//      #[test]
//     fn unit_test_erase_non_exist() {
//         let skiplist = Skiplist::new();
//         let rst = skiplist.erase(9);
//         assert_eq!(rst, false);
//     }

//     #[test]
//     fn unit_test_erase() {
//         let skiplist = Skiplist::new();
//         skiplist.add(9);
//         let rst = skiplist.erase(9);
//         assert_eq!(rst, true);
//     }

//     #[test]
//     fn unit_test_complex() {
//         let skiplist = Skiplist::new();
//         skiplist.add(1);
//         //skiplist.print();
//         skiplist.add(2);
//         //skiplist.print();
//         skiplist.add(3);
//         //skiplist.print();
//         let rst = skiplist.search(0);
//         assert_eq!(rst, false);
//         skiplist.add(4);
//         let rst = skiplist.search(1);
//         assert_eq!(rst, true);
//         let rst = skiplist.erase(0);
//         assert_eq!(rst, false);
//         let rst = skiplist.erase(1);
//         assert_eq!(rst, true);
//         skiplist.print();
//         let rst = skiplist.search(1);
//         assert_eq!(rst, false);
//     }

// }
