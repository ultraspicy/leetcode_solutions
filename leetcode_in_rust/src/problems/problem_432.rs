use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

#[derive(Debug, Default)]
#[allow(dead_code)]
struct AllOne {
    head: Option<Rc<RefCell<Count>>>,
    tail: Option<Rc<RefCell<Count>>>,

    string_frequency: HashMap<String, i32>,
    frequency_count: HashMap<i32, Rc<RefCell<Count>>>,
    frequency_string: HashMap<i32, HashSet<String>>
}

// Default for a self-referential structure creates infinite recursion. 
// need to build the structure step by step instead.
#[derive(Debug, Default)]
#[allow(dead_code)]
struct Count {
    val: i32,
    prev: Option<Weak<RefCell<Count>>>,
    next: Option<Rc<RefCell<Count>>>,
}

#[allow(dead_code)]
impl AllOne {

    fn new() -> Self {
        let head = Rc::new(RefCell::new(Count{val: -1, ..Default::default()}));
        let tail = Rc::new(RefCell::new(Count{val: -1, ..Default::default()}));
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));
        AllOne { 
            head: Some(head),
            tail: Some(tail),
            ..Default::default()
        }
    }

    fn add_node(&mut self, prev: Option<Rc<RefCell<Count>>>, next: Option<Rc<RefCell<Count>>>, freq: i32) {
        let count = Rc::new(RefCell::new(Count{val: freq, ..Default::default()}));
        count.borrow_mut().next = next.clone();
        count.borrow_mut().prev = Some(Rc::downgrade(&prev.clone().unwrap()));
        prev.unwrap().borrow_mut().next = Some(count.clone());
        next.unwrap().borrow_mut().prev = Some(Rc::downgrade(&count));
        
        self.frequency_count.insert(freq, count);
    }

    fn remove_node(&mut self, freq: i32) {
        let cur = self.frequency_count.get(&freq).unwrap().clone();
        let prev = cur.borrow().prev.clone().unwrap().upgrade().unwrap();
        let next = cur.borrow().next.clone();
        prev.borrow_mut().next = next.clone();
        next.unwrap().borrow_mut().prev = Some(Rc::downgrade(&prev));

        self.frequency_count.remove(&freq);
    }
    
    fn inc(&mut self, key: String) {
        let freq = self.string_frequency.get(&key).copied().unwrap_or_default();
        *self.string_frequency.entry(key.clone()).or_insert(0) += 1;

        match freq {
            0 => {
                let freq_plus_one = freq + 1;
                if !self.frequency_count.contains_key(&freq_plus_one) {
                    let prev = Some(self.head.clone().unwrap());
                    let next = self.head.clone().unwrap().borrow().next.clone();
                    self.add_node(prev, next, freq_plus_one);
                }
                self.frequency_string.entry(freq_plus_one).or_insert_with(HashSet::new).insert(key.clone());
            },
            freq => {
                let freq_plus_one = freq + 1;
                if !self.frequency_count.contains_key(&freq_plus_one) {
                    let prev: Rc<RefCell<Count>> = self.frequency_count.get(&freq).unwrap().clone();
                    let next = prev.borrow().next.clone();
                    self.add_node(Some(prev), next, freq_plus_one);
                   
                }
                self.frequency_string.entry(freq_plus_one).or_insert_with(HashSet::new).insert(key.clone());
                self.frequency_string.get_mut(&freq).unwrap().remove(&key);

                if self.frequency_string.get(&freq).unwrap().len() == 0 {
                    self.frequency_string.remove(&freq);
                    self.remove_node(freq);
                }

            },
        }
    }
    
    fn dec(&mut self, key: String) {
        let freq = self.string_frequency.get(&key).copied().unwrap_or_default();
        *self.string_frequency.get_mut(&key).unwrap() -= 1;
        match freq {
            1 => {
                self.frequency_string.get_mut(&1).unwrap().remove(&key);
                if self.frequency_string.get(&1).unwrap().len() == 0 {
                    self.frequency_string.remove(&1);
                    self.remove_node(1);
                }
            },
            freq => {
                let freq_minus_one = freq - 1;
                if !self.frequency_count.contains_key(&freq_minus_one) {
                    let next: Rc<RefCell<Count>> = self.frequency_count.get(&freq).unwrap().clone();
                    let prev = next.borrow().prev.clone().unwrap().upgrade();
                    self.add_node(prev, Some(next), freq_minus_one);
                }
                self.frequency_string.entry(freq_minus_one).or_insert_with(HashSet::new).insert(key.clone());
                self.frequency_string.get_mut(&freq).unwrap().remove(&key);

                if self.frequency_string.get(&freq).unwrap().len() == 0 {
                    self.frequency_string.remove(&freq);
                    self.remove_node(freq);
                }
            
            },
        }
    }
    
    fn get_max_key(&self) -> String {
       
        let prev = self.tail.clone().unwrap().borrow().prev.clone();
        match prev {
            Some(prev) => {
                let freq = prev.clone().upgrade().unwrap().borrow().val;
                if freq == -1 {
                    return String::from("")
                }
                self.frequency_string.get(&freq).unwrap().iter().next().unwrap().clone()
            },
            None => { String::from("") },
        }

    }
    
    fn get_min_key(&self) -> String {
        let next = self.head.clone().unwrap().borrow().next.clone();
        match next {
            Some(next) => {
                let freq = next.borrow().val;
                if freq == -1 {
                    return String::from("")
                }
                self.frequency_string.get(&freq).unwrap().iter().next().unwrap().clone()
            },
            None => { String::from("") },
        }
    }

    fn print(&self) {
        println!("=============================");
        println!("{:?}", self.string_frequency); 
        println!("{:?}", self.frequency_string);
        println!("head.next = {}", self.head.clone().unwrap().borrow().next.clone().unwrap().borrow().val);
        println!("tail.prev = {}", self.tail.clone().unwrap().borrow().prev.clone().unwrap().upgrade().unwrap().borrow().val);
    }
}

#[cfg(test)]
mod test {
    use crate::problems::problem_432::AllOne;


    #[test]
    fn test_allone_sequence() {
        let mut all_one = AllOne::new();
        
        // ["AllOne","inc","inc","inc","inc","getMaxKey","inc","inc","inc","dec","inc","inc","inc","getMaxKey"]
        // [[],["hello"],["goodbye"],["hello"],["hello"],[],["leet"],["code"],["leet"],["hello"],["leet"],["code"],["code"],[]]
        
        // inc("hello") - hello: 1
        all_one.inc("hello".to_string());
        
        
        // inc("goodbye") - hello: 1, goodbye: 1
        all_one.inc("goodbye".to_string());
        
        // inc("hello") - hello: 2, goodbye: 1
        all_one.inc("hello".to_string());
        
        // inc("hello") - hello: 3, goodbye: 1
        all_one.inc("hello".to_string());
        
        
        // getMaxKey() - should return "hello" (freq 3)
        assert_eq!("hello", all_one.get_max_key());
        all_one.print();
        
        // inc("leet") - hello: 3, goodbye: 1, leet: 1
        all_one.inc("leet".to_string());
        
        // inc("code") - hello: 3, goodbye: 1, leet: 1, code: 1
        all_one.inc("code".to_string());
        
        // inc("leet") - hello: 3, goodbye: 1, leet: 2, code: 1
        all_one.inc("leet".to_string());
        
        
        // dec("hello") - hello: 2, goodbye: 1, leet: 2, code: 1
        all_one.dec("hello".to_string());
        
        
        // inc("leet") - hello: 2, goodbye: 1, leet: 3, code: 1
        all_one.inc("leet".to_string());
        
        // inc("code") - hello: 2, goodbye: 1, leet: 3, code: 2
        all_one.inc("code".to_string());
        
        // inc("code") - hello: 2, goodbye: 1, leet: 3, code: 3
        all_one.inc("code".to_string());

        // all_one.print();
        
        // getMaxKey() - should return either "leet" or "code" (both freq 3)
        let max_key = all_one.get_max_key();
        assert!(max_key == "leet" || max_key == "code", "Expected 'leet' or 'code', got '{}'", max_key);

         let min_key = all_one.get_min_key();
         assert_eq!("goodbye", min_key);
        
        println!("All tests passed!");
    }
}