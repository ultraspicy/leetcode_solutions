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

#[cfg(test)]
mod application {
    use super::*;
    use std::{sync::{Arc, Mutex}, thread};

    #[test]
    fn multithread_mutex() -> Result<(), Box<dyn std::error::Error>> {
        let time_map_rc = Arc::new(Mutex::new(TimeMap::new()));

        let guard_rc1 = time_map_rc.clone();
        let h1 = thread::spawn( move || -> Result<(), String>  { // 'static means the closure contains no borrowed references //  The spawned thread can outlive the function that spawned it
            let mut guard = guard_rc1.lock().map_err(|e| e.to_string())?; // MutexGuard implements DerefMut, so you can get &mut T through it. 
            guard.set(String::from("foo"), String::from("bar"), 1);
            Ok(())
        });

        let guard_rc2  = time_map_rc.clone();
        let h2 = thread::spawn( move || -> Result<(), String>  {
            let mut guard = guard_rc2.lock().map_err(|e| e.to_string())?;
            guard.set(String::from("foo"), String::from("bar2"), 2);
            Ok(())
        });

        h1.join()
            .map_err(|e| {
                if let Some(str) = e.downcast_ref::<&str>() { str.to_string()} else {"h1 panics".to_string()}
            })?
            .map_err(|e| -> Box<dyn std::error::Error> {e.into()})?;
        h2.join()
            .map_err(|e| {
                if let Some(str) = e.downcast_ref::<&str>() { str.to_string()} else {"h1 panics".to_string()}
            })?
            .map_err(|e| -> Box<dyn std::error::Error> {e.into()})?;
        Ok(())
    }
}

// follow up: Concurrent Solution
// 1. Arc<RwLock<TimeMap>> or Arc<Mutex<TimeMap>>
//   Mutex is simple — only one thread at a time. RwLock allows multiple concurrent readers
// 2. Sharding the map into N independent buckets by key hash
// 3. Lock-free structure — use DashMap (a popular crate) which handles concurrent access internally with fine-grained locking
// 4. Actor model — put TimeMap in a single-threaded task/actor, communicate via channels (mpsc). No locks at all, serialized access through message passing
// 5. (R >> W) Immutable snapshots + copy-on-write — use Arc<BTreeMap> and atomically swap the pointer on writes; readers clone the Arc and read without locking.
//   Works well when reads vastly outnumber writes. This is a generic strategy for trading read throught with write complexity

// Mutex vs RwLock

// MutexWrite-heavy workloads — if you're mostly writing, RwLock's overhead of tracking reader counts etc. is wasted. 
// Every thread waiting for a Mutex is in the same queue, regardless of what they want to do with the data
// 
// Simplicity/correctness — RwLock has a subtle failure mode called writer starvation, where a flood of readers can indefinitely block a waiting writer. Mutex has no such issue
// Raw overhead — Mutex has a simpler internal structure, so per-lock-operation it's slightly cheaper
// A writer has to wait for all current readers to finish. But while it's waiting, new readers can keep arriving and being let in

// follow up: (de)serialization