
use std::collections::{BinaryHeap, HashMap};

struct TaskManager {
    tasks: HashMap<i32, (i32, i32)>,
    heap: BinaryHeap<(i32, i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl TaskManager {

    fn new(tasks: Vec<Vec<i32>>) -> Self {
        // [userId, taskId, priority]
        let mut m = HashMap::new();
        let mut h = BinaryHeap::new();
        tasks.into_iter().for_each(|t| {
            let (user_id, task_id, priority) = (t[0], t[1], t[2]);
            m.insert(task_id, (priority, user_id));
            h.push((priority, task_id, user_id));
        });
        Self { tasks: m, heap: h }
    }
    
    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.insert(task_id, (priority, user_id));
        self.heap.push((priority, task_id, user_id));
    }
    
    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some(&(_, user_id)) = self.tasks.get(&task_id) {
            self.tasks.insert(task_id, (new_priority, user_id));
            self.heap.push((new_priority, task_id, user_id));
        }
    }
    
    fn rmv(&mut self, task_id: i32) {
        self.tasks.remove(&task_id);
        
    }
    
    fn exec_top(&mut self) -> i32 {
        while let Some((priority, task_id, user_id)) = self.heap.pop() {
          
            if let Some(&(latest_prioriy, latest_user_id)) = self.tasks.get(&task_id) {
                if latest_prioriy != priority || latest_user_id != user_id {
                    continue;
                } else {
                    self.rmv(task_id);
                    return user_id;
                }
            } else {
                continue;
            }
        }

        -1
    }
}
