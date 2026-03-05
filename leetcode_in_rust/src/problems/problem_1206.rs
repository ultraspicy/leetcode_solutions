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
    before: Option<WeakRef<T>>,
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
            before: Option::default(),
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
        (0..3).for_each(|i| {
            let cur = heads[i].clone().unwrap();
            let below = heads[i+1].clone().unwrap();
            cur.borrow_mut().below = Some(below.clone());
            below.borrow_mut().above = Some(Rc::downgrade(&cur.clone()));
        });
        Skiplist { heads:heads }
    }

    fn search(&self, target: i32) -> bool {
        let candidate = self.search_internal(target);
        if let Some(node) = candidate {
            return node.borrow().val == target
        }
        false
    }

    fn add(&mut self, num: i32) {
        // go to the target level first
        let mut cursor = self.heads[0].clone().unwrap();
        let mut r = rand::rng();
        for _ii in 0..3 {
            if r.random::<bool>() && r.random::<bool>() {
                break;
            } else {
                let below_rc = cursor.borrow().below.clone().unwrap();
                cursor = below_rc;
            }
        }

        let mut prev :Option<NodeRef<i32>> = None;
        loop {
            // search the last item <= num, then add itself and all node below
            loop {
                let next_rc = cursor.borrow().next.clone();
                match next_rc {
                    Some(next) => {
                        if next.borrow().val <= num {
                            cursor = next;
                        } else {
                            break;
                        }
                    },
                    None => { break },
                }
            }
            let node = ListNode::new_rc(num);
            let next = cursor.borrow().next.clone();
            cursor.borrow_mut().next = Some(node.clone());
            node.borrow_mut().before = Some(Rc::downgrade(&cursor.clone()));
            node.borrow_mut().next = next.clone();
            if let Some(next_rc) = next {
               next_rc.borrow_mut().before = Some(Rc::downgrade(&node.clone()));
            }
            //node.borrow_mut().above = prev;
            if let Some(prev_rc) = prev {
                prev_rc.borrow_mut().below = Some(node.clone());
                node.borrow_mut().above = Some(Rc::downgrade(&prev_rc));
            }

            let below_rc = cursor.borrow().below.clone();
            match below_rc {
                Some(below) => {cursor = below},
                None => {break}
            }
            prev = Some(node.clone());
        }
    }

    fn erase(&mut self, num: i32) -> bool {
        // search_internal get the node
        let mut candidate = self.search_internal(num);
        if candidate.clone().is_none() {
            return false;
        }
        if candidate.as_ref().unwrap().borrow().val != num {
            return false;
        }

        // remove
        while let Some(candidate_rc) = candidate {
            let before = candidate_rc.borrow().before.clone();
            let next = candidate_rc.borrow().next.clone(); // next could be None
            if let Some (before_rc) = before.clone().and_then(|wk|wk.upgrade()) {
                before_rc.borrow_mut().next = next.clone();
            }
            if let Some(next_rc) = next {
                next_rc.borrow_mut().before = before.clone();
            }
            candidate = if let Some(above_wk) = candidate_rc.borrow().above.clone() {
                above_wk.upgrade()
            } else {
                None
            }
        }

        true
    }

    fn search_internal(&self, target: i32) -> Option<NodeRef<i32>> {
        let mut cursor = self.heads[0].clone();
        (0..4).for_each(|_lvl| {
            while let Some(cur) = cursor.clone() {
                if let Some(next) = cur.borrow().next.clone() {
                    let val = next.borrow().val;
                    if val <= target {
                        cursor = Some(next.clone());
                        print!("move right >>> ");
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if let Some(cur_rc) = cursor.clone() {
                if let Some(below_rc) = cur_rc.borrow().below.clone() {
                    cursor = Some(below_rc);
                    print!("move down >>> ");
                }
            }
        });

        cursor
    }

    fn print(&self) {
        for (lvl, head) in self.heads.iter().enumerate() {
            print!("level {}: ", lvl);
            let mut cursor = head.clone();
            while let Some(cur_rc) = cursor {
                print!("{} -> ", cur_rc.borrow().val);
                cursor = cur_rc.borrow().next.clone();
            }
            println!("None");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ntest::timeout;

    fn construct_list() -> Skiplist {
        let mut l = Skiplist::new();
        l.add(1);
        l.add(3);
        l.add(5);
        l.add(7);
        l.add(9);
        l.add(9);

        l
    }

    #[test]
    fn setup_skiplist() {
        let mut l = Skiplist::new();
        l.add(1);
        l.add(3);
        l.add(5);
        l.add(7);
        l.add(9);
        l.add(9);
    }

    #[test]
    #[timeout(300)]
    fn test_search_internal() {
        let l = construct_list();
        let ret = l.search_internal(9);
        assert!(ret.is_some());

        let ret = l.search_internal(5);
        assert!(ret.is_some());

        let ret = l.search_internal(2);
        assert!(ret.is_some());
        assert_eq!(1, ret.unwrap().borrow().val);
    }

    #[test]
    #[timeout(500)]
    fn test_search() {
        let l = construct_list();
        let ret = l.search(9);
        assert!(ret);

        let ret = l.search(5);
        assert!(ret);

        let ret = l.search(2);
        assert!(!ret);
    }

    #[test]
    #[timeout(500)]
    fn test_erase() {
        let l = construct_list();
        let ret = l.search(9);
        assert!(ret);

        let ret = l.search(9);
        assert!(ret);

        let ret = l.search(2);
        assert!(!ret);
    }

    #[test]
    fn lc_test_case_1() {
        let mut l = Skiplist::new();
        l.add(1);
        l.add(2);
        l.add(3);
        l.add(4);

        let ret = l.search_internal(1);
        assert_eq!(1, ret.unwrap().borrow().val);
        let ret = l.search(1);
        assert!(ret);

        let ret = l.erase(0);
        assert!(!ret);
    }

    #[test]
    fn lc_test_case_2() {
        let mut l = Skiplist::new();
        l.add(1);
        l.add(2);
        l.add(3);

        let ret = l.search(0);
        assert!(!ret);
        l.add(4);
        l.print();
        let ret = l.search(1);
        assert!(ret);
        l.print();

        //let ret = l.search_internal(1);
        // assert_eq!(1, ret.unwrap().borrow().val);
        // let ret = l.search(1);
        // assert!(ret);

        // let ret = l.erase(0);
        // assert!(!ret);
    }
}
