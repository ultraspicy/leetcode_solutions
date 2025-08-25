use std::collections::BTreeMap;

struct MaxStack {
    index_to_number: BTreeMap<usize, i32>,
    num_to_indexes: BTreeMap<i32, Vec<usize>>,
    size: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxStack {

    fn new() -> Self {
        MaxStack { index_to_number: BTreeMap::new(), num_to_indexes: BTreeMap::new(), size: 0 } 
    }
    
    fn push(&mut self, x: i32) {
        // insert index
        // update ordered_map, (insert_key) and insert_index
        // size ++
        self.index_to_number.insert(self.size, x);
        self.num_to_indexes.entry(x).or_insert_with(Vec::new).push(self.size);
        self.size += 1;
    }
    
    fn pop(&mut self) -> i32 {
        // remove last element from linear_vec
        // remove its index from ordered_map as well
        // size--
        let entry = self.index_to_number.last_entry();
        let number = match entry {
            Some(occupied) => {
                let (_index, value) = occupied.remove_entry();
                value
            },
            None => { -1 },
        };

        if let Some(indice_list) = self.num_to_indexes.get_mut(&number) {
            indice_list.pop();
            if indice_list.is_empty() {
                self.num_to_indexes.remove(&number);
            }
        }

        self.size -= 1;

        number
    }
    
    fn top(&self) -> i32 {
        // read val of the last element
        if let Some((&_k, &v)) = self.index_to_number.last_key_value() {
            v
        } else {
            -1
        }
    }
    
    fn peek_max(&mut self) -> i32 {
        // get max from ordered_map
        if let Some(occupied) = self.num_to_indexes.last_entry() {
            return *occupied.key()
        }

        -1
    }
    
    fn pop_max(&mut self) -> i32 {
        // get max from ordered_map
        // remove key, remove index
        // linear_vec update as None

        let mut last_entry = self.num_to_indexes.last_entry().unwrap();
        let ret = *last_entry.key();
        let index_list= last_entry.get_mut();
        let index = index_list.pop().unwrap_or_else(|| panic!("index_list is empty???"));
        if index_list.is_empty() {
            last_entry.remove_entry();
        } 

        self.index_to_number.remove(&index);
        ret
    }
    
    fn print(&self) {
        println!("index_to_number = {:?}", self.index_to_number);
        println!("size = {}", self.size);
    }
}

#[cfg(test)]
mod test {
    use crate::problems::problem_716::MaxStack;


    #[test]
    fn unit_test (){
        let mut s = MaxStack::new();
        s.push(5);
        s.push(1);
        s.push(5);
        s.print();
        let ret = s.top();
        assert_eq!(5, ret);
        let ret = s.pop_max();
        assert_eq!(5, ret);
        let ret = s.top();
        assert_eq!(1, ret);
        let ret = s.peek_max();
        assert_eq!(5, ret);
        let ret = s.pop();
        assert_eq!(1, ret);
        let ret = s.top();
        assert_eq!(5, ret);
    }
}