use std::{collections::HashMap, usize};

struct LRUCache {
    map: HashMap<i32, usize>,
    capacity: i32,
    arena: Vec<Node>,
}

#[derive(Clone, Copy, Debug)]
struct Node {
    index: usize,
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

impl Node {
    fn new(idx: usize, key: i32, value: i32, prev: usize, next: usize) -> Self {
        Self {
            index: idx,
            key: key,
            value: value,
            prev: prev,
            next: next,
        }
    }
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl LRUCache {

    fn new(capacity: i32) -> Self {
        // init arena, create dummy_head at idx_0, dummy_tail as idx_1
        let dummy_head = Node::new(0, -1, -1, usize::MAX, 1);
        let dummy_tail = Node::new(1, -1, -1, 0, usize::MAX);
        let arena = vec![dummy_head, dummy_tail];
        
        Self { 
            map: HashMap::new(), 
            capacity: capacity, 
            arena: arena,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        Self::move_front(self, key)
    }
    
    fn put(&mut self, key: i32, new_value: i32) {
        // if key exist 
        //   if value not same
        //     update the node value
        //.  move key to the front
        // else 
        //   create key, put key to fron.
        //   if at_capacity
        //     evict last
        if self.map.contains_key(&key) {
            self.arena[self.map.get_mut(&key).copied().unwrap()].value = new_value;
            Self::move_front(self, key);
        } else {
            let current_first = self.arena[0].next;
            // let new_node = Node::new(self.arena.len(), new_value, 0, current_first);
            self.arena[current_first].prev = self.arena.len();
            self.arena[0].next = self.arena.len();
            let new_head = Node::new(self.arena.len(), key, new_value, 0, current_first);
            
            self.map.insert(key, self.arena.len());
            self.arena.push(new_head);
            
            if self.arena.len() > (self.capacity + 2) as usize {
                // evict last
                let last_idx = self.arena[1].prev;
                let prev_of_last = self.arena[last_idx].prev;
                self.arena[prev_of_last].next = 1;
                self.arena[1].prev = prev_of_last;
                
                self.map.remove(&self.arena[last_idx].key);
            }
            
        }
    }

    fn move_front(&mut self, key: i32) -> i32 {
        // if map has key 
        //   move key to the front
        //.  return value
        // else 
        //   return -1 
        if !self.map.contains_key(&key) {
            return -1;
        }
        let node = self.arena[self.map.get(&key).copied().expect("key guaranteed by contains_key check")];
        let (prev_idx, next_idx) = (node.prev, node.next);
        let val = node.value;
        // remove node from original position
        self.arena[prev_idx].next = next_idx;
        self.arena[next_idx].prev = prev_idx;
        // put new head in front
        let current_first = self.arena[0].next;
        self.arena[current_first].prev = node.index;
        self.arena[0].next = node.index;
        self.arena[node.index].prev = 0;
        self.arena[node.index].next = current_first;

        val
    }

    fn print(&self) {
        let mut cur_idx = 0;
        while cur_idx != usize::MAX {
            print!("({}, {}) -> ", self.arena[cur_idx].key, self.arena[cur_idx].value);
            cur_idx = self.arena[cur_idx].next;
        }
        println!(" // end");
        println!("self.map = {:?}", self.map)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test() {
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        // lru.print();
        lru.put(2, 2);
        // lru.print();
        let rst = lru.get(1);
        assert_eq!(1, rst);
        //lru.print();
        lru.put(3,3);
        lru.print();
        let rst = lru.get(2);
        lru.print();
        assert_eq!(-1, rst);
        lru.print();
        lru.put(4,4);
        lru.print();
    }
}
