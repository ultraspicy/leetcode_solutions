use std::usize;

use rand::Rng;

struct Skiplist {
    level: usize,
    arena: Vec<Node>,
}

#[derive(Default, Clone, Copy, Debug)]
struct Node {
    val: i32,

    idx: usize,
    next: usize,
    prev: usize,
    below: usize,
    above: usize,
}


impl Node {
    fn new(val: i32, idx: usize) -> Self {
        Node { val, idx: idx, next: usize::MAX, prev: usize::MAX, below: usize::MAX, above: usize::MAX}
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Skiplist {

    fn new() -> Self {
        let level = 4;
        let mut heads= vec![];

        for i in 0..level {
            let head = Node::new(-1, i);
            heads.push(head);
            if i > 0 {
                heads[i - 1].below = i;
                heads[i].above = i - 1;
            }
        }
        return Skiplist {
            arena: heads,
            level: level,
        };
    }

    fn search(&self, target: i32) -> bool {
        self.search_internal(target) != usize::MAX
    }

    fn add(&mut self, num: i32) {
        // go to the dedicated level first
        // loop:
        //   go right to find the prev_node for insertion, insert
        //   go prev_node.next
        let space = usize::MAX;
        let mut start_rng = space / (2usize.pow((self.level - 1) as u32));
        let mut cursor_idx = 0;
        let mut rng = rand::thread_rng();

        while rng.gen_range(0..space) > start_rng {
            cursor_idx = self.arena[cursor_idx].below;
            start_rng = start_rng * 2;
        }

        let mut above_level_node_idx = usize::MAX;
        loop {
            while self.arena[cursor_idx].next != usize::MAX {
                let next = self.arena[cursor_idx].next;
                if self.arena[next].val < num {
                    cursor_idx = next;
                } else {
                    break;
                }
            }
            // add node
            let mut new_node = Node::new(num, self.arena.len());

            if self.arena[cursor_idx].next != usize::MAX {
                let next = self.arena[cursor_idx].next;
                new_node.next = next;
                self.arena[next].prev = new_node.idx;
            }
            self.arena[cursor_idx].next = new_node.idx;
            new_node.prev = self.arena[cursor_idx].idx;

            if above_level_node_idx != usize::MAX {
                self.arena[above_level_node_idx].below = new_node.idx;
                new_node.above = above_level_node_idx;
            }

            self.arena.push(new_node);
            above_level_node_idx = new_node.idx;

            // go below
            if self.arena[cursor_idx].below == usize::MAX { break; }
            cursor_idx = self.arena[cursor_idx].below;
        }
    }

    fn erase(&mut self, num: i32) -> bool {
        // search the node in skip list
        // if true:
        //   remove node and all its copies
        // if false
        //   false, no need to erase
        let mut idx = self.search_internal(num);
        if idx == usize::MAX {
            return false;
        }
        while idx != usize::MAX {
            let next_idx = self.arena[idx].next;
            let prev_idx  = self.arena[idx].prev;
            self.arena[prev_idx].next = next_idx;
            if next_idx != usize::MAX {
                self.arena[next_idx].prev = prev_idx;
            }
            idx = self.arena[idx].below;
        }

        true
    }

    fn search_internal(&self, target: i32) -> usize {
        // loop:
        //   go right to find the prev_node
        //.  if prev.node.next.val == target
        //     return true
        //.  else
        //.    go prev_node.below
        let mut cursor_idx = 0;
        while cursor_idx != usize::MAX {
            while self.arena[cursor_idx].next != usize::MAX { //&& self.arena[cursor.next].val < target {
                let next = self.arena[cursor_idx].next;
                if self.arena[next].val < target {
                    cursor_idx = next;
                } else {
                    break;
                }
            }
            if self.arena[cursor_idx].next == usize::MAX || self.arena[self.arena[cursor_idx].next].val > target {
                cursor_idx = self.arena[cursor_idx].below;
            } else {
                return self.arena[cursor_idx].next;
            }
        }

        usize::MAX
    }

    fn print(&self) {
        println!("{:?}", self.arena);
        for i in 0..self.level {
            let mut cursor = self.arena[i];
            print!("{}", cursor.val);
            while cursor.next != usize::MAX {
                cursor = self.arena[cursor.next];
                print!(" -> {}", cursor.val);
            }
            println!("");
        }

    }
}

#[cfg(test)]
mod test {
    use core::time;

    use super::*;
    use ntest::timeout;

    #[test]
    #[timeout(300)]
    fn unit_test_1() {
        let mut l = Skiplist::new();
        l.add(1);
        l.add(2);
        l.add(3);
        l.print();

        let rst = l.search(0);
        assert_eq!(false, rst);
        l.add(4);
        let rst =  l.search(1);
        assert_eq!(true, rst);

        let rst =  l.add(5);
        let rst =  l.search(3);
        assert_eq!(true, rst);
        let rst =  l.search(6);
        assert_eq!(false, rst);
    }

    #[test]
    #[timeout(300)]
    fn unit_test__2() {
        let mut l = Skiplist::new();
        l.add(9);
        l.add(4);
        l.add(5);
        l.add(6);
        l.add(9);
        l.print();

        let rst = l.erase(2);
        assert_eq!(false, rst);
        let rst = l.erase(1);
        assert_eq!(false, rst);
        l.add(2);
        l.print();
        let rst =  l.search(7);
        assert_eq!(false, rst);
        let rst =  l.search(4);
        assert_eq!(true, rst);
        // l.add(4);
        // let rst =  l.search(1);
        // assert_eq!(true, rst);

        // let rst =  l.add(5);
        // let rst =  l.search(3);
        // assert_eq!(true, rst);
        // let rst =  l.search(6);
        // assert_eq!(false, rst);
    }
}
