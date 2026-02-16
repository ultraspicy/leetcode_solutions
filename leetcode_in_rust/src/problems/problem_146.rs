
use std::{cell::RefCell, collections::HashMap};
use std::rc::{Rc, Weak};

#[allow(dead_code)]
struct LRUCache {
    capacity: i32,
    size: i32,
    kv_store: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}
#[allow(dead_code)]
#[derive(Debug, Default, Clone,)]
struct Node {
    key: i32,
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

#[allow(dead_code)]
impl Node {
    fn new (key: i32, val: i32) -> Self {
        Node {
            key: key,
            val: val,
            ..Default::default()
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
        let head = Rc::new(RefCell::new(Node { val: -1, ..Default::default()}));
        let tail = Rc::new(RefCell::new(Node { val: -1, ..Default::default()}));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));

        LRUCache { 
            capacity: capacity, 
            size: 0,
            kv_store: HashMap::new(), 
            head: head,
            tail: tail,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        // check hashmap
        // if exist, move key to the top, then return
        // if non-exist, return -1
        let v = self.kv_store.get(&key).cloned(); // Option<&T>.cloned() -> Option<T>
        match v {
            Some(val) => {
                self.remove_node(val.clone());
                let value = val.borrow().val;
                self.add_first(key, value);
                value
            },
            None => { -1 }
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        // if exist
        //.   move key to the top
        //.else 
        //.   if not_at_capacity
        //.      add key to map, add node to DLL, size++
        //.   else 
        //.      remove the last_node in DLL, remove last_entry from hashmap, add new_node to the top, add new_entry
        if self.kv_store.contains_key(&key) {
            let node_rc = self.kv_store.get(&key).unwrap().clone();
            self.remove_node(node_rc);
            self.add_first(key, value);
        } else {
            if self.size < self.capacity {
                self.add_first(key, value);
                self.size += 1;
            } else {
                let _removed = self.remove_last().unwrap();
                self.add_first(key, value);
            }
        }
    }

    fn add_first(&mut self, key: i32, val: i32) -> Option<Rc<RefCell<Node>>> {
        let first = self.head.borrow().next.clone();
        let new_node = Rc::new(RefCell::new(Node::new(key, val)));
        match first {
            Some(next_node) => {
                new_node.borrow_mut().next = Some(next_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&self.head));
                self.head.borrow_mut().next = Some(new_node.clone());
                next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            },
            None => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&self.head));
                self.head.borrow_mut().next = Some(new_node.clone());
            },
        }
        
        self.kv_store.insert(key, new_node)
    }

    fn remove_node(&mut self, node: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        let prev = node.borrow().prev.clone()?;
        let next = node.borrow().next.clone()?;
        prev.upgrade()?.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
        
        self.kv_store.remove(&node.borrow().key)
      
    }

    fn remove_last(&mut self) -> Option<Rc<RefCell<Node>>> {
        let tail_prev = self.tail.borrow().prev.clone()?;
        let last_rc = tail_prev.upgrade()?;
        if last_rc.borrow().val != -1 {
            let node_prev = last_rc.borrow().prev.clone()?.upgrade()?;
            node_prev.borrow_mut().next = Some(self.tail.clone());
            self.tail.borrow_mut().prev = Some(Rc::downgrade(&node_prev));
        }

        let k = last_rc.borrow().key; 
        self.kv_store.remove(&k)
    }
}
