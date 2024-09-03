
use super::Solution;

use std::collections::HashMap;

// key learnings:
//   entry() method returns an Entry enum, which represents a slot in the HashMap with its status (occupied or vacant)
//   or_insert() method returns a mutable reference of V 
//   iter() yeilds (&K, &V), when matching the pattern of (&k, &v) This pattern dereferences both references.|(k, v)|:
//   This pattern keeps the references as they are.
//     k is a reference to the key.
//     v is a reference to the value.


//   map.iter_mut(): Returns an iterator over mutable references to the values.
//   map.into_iter(): Consumes the map and returns an iterator over owned key-value pairs. 

impl Solution {
    pub fn winning_player_count(_n: i32, picks: Vec<Vec<i32>>) -> i32 {
        let mut map:HashMap<i32, HashMap<i32, i32>> = HashMap::new();
        for pick in picks {
            let color_frequency = map.entry(pick[0]).or_insert(HashMap::new());
            *color_frequency.entry(pick[1]).or_insert(0) += 1; 
        }

        let win = map.iter().
            filter(
                |(&k, v)| v.values().max().map_or(false, |&max| max > k)).
            count() as i32;

        win
    } 
}  