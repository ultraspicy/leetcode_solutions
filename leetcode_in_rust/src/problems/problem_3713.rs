use std::collections::HashMap;

use super::Solution;

impl Solution {
    // TODO: Implement the method for problem 3713
    // pub fn method_name(params) -> return_type {
    //     unimplemented!()
    // }

    pub fn longest_balanced(s: String) -> i32 {

        if s.len() == 0 {
            return 0;
        }

        let idx: HashMap<char, Vec<usize>> = s.char_indices()
            .fold(HashMap::new(), |mut acc, (i, c)| {
                acc.entry(c).or_insert(Vec::new()).push(i);
                acc
            });
        
        let len = s.len();
        let mut ret = 1;

        println!("idx = {:?}", idx);
        
        (0..len).for_each(|start|{
            ((start + 1)..=len).for_each(|end| {
                if Self::is_slice_balanced(&idx, start, end) {
                    ret = ret.max((end - start) as i32);
                }
            });
        });

        ret
    }

    fn is_slice_balanced(idx: &HashMap<char, Vec<usize>>, start: usize, end: usize) -> bool {
        let mut freq = -1;
        let alphabet =  "abcdefghijklmnopqrstuvwxyz";

        
        for c in alphabet.chars() {
            // if idx has no c, then skip
            // if idx has c, counter for the appearence
            if let Some(position) = idx.get(&c) {
                let first_partition = position.partition_point(|&x| x < start);
                let second_partition = position.partition_point(|&x| x < end);
                println!("c = {}, start = {}, end = {}, p1 = {}, p2 = {}", c, start, end, first_partition, second_partition);

                if first_partition != second_partition  {
                    if freq == -1 {
                        freq = (second_partition - first_partition) as i32;
                    } else {
                        if (second_partition - first_partition) as i32 != freq {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn unit_test() {
        println!("{}", Solution::longest_balanced(String::from("jhj")));
    } 
}