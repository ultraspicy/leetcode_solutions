use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::*;

#[allow(dead_code)]
struct TimeMap {
    map: HashMap<String, BTreeMap<i32, String>>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl TimeMap {

    fn new() -> Self {
        Self{
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(tree_map) = self.map.get(&key) {
            if let Some((_k, v)) = tree_map.range((Unbounded, Included(&timestamp))).next_back() {
                v.clone()
            } else {
                String::from("")
            }
        } else {
            String::from("")
        }
    }
}

// follow up: Concurrent Solution
// 1. Arc<RwLock<TimeMap>> or Arc<Mutex<TimeMap>>
// 2. Sharding the map into N independent buckets by key hash
// 3. Lock-free structure — use DashMap (a popular crate) which handles concurrent access internally with fine-grained locking
// 4. Actor model — put TimeMap in a single-threaded task/actor, communicate via channels (mpsc). No locks at all, serialized access through message passing
// 5. (R >> W) Immutable snapshots + copy-on-write — use Arc<BTreeMap> and atomically swap the pointer on writes; readers clone the Arc and read without locking.
//   Works well when reads vastly outnumber writes. This is a generic strategy for trading read throught with write complexity

// follow up: (de)serialization
