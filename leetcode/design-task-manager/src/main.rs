// https://leetcode.com/problems/design-task-manager/?envType=daily-question&envId=2025-09-18

#[derive(Clone, PartialEq, Eq)]
struct Task {
    user: i32,
    id: i32,
    priority: i32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority
            .cmp(&other.priority)
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

use std::collections::BTreeSet;
use std::collections::HashMap;
struct TaskManager {
    tasks: HashMap<i32, Task>,
    sorted_tasks: BTreeSet<Task>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let tasks: HashMap<i32, Task> = tasks.into_iter().fold(HashMap::new(), |mut map, task| {
            let (user_id, task_id, priority) = (task[0], task[1], task[2]);
            map.insert(
                task_id,
                Task {
                    user: user_id,
                    id: task_id,
                    priority,
                },
            );
            map
        });
        let sorted_tasks = BTreeSet::from_iter(tasks.values().cloned());
        Self {
            tasks,
            sorted_tasks,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        let task = Task {
            user: user_id,
            id: task_id,
            priority,
        };
        self.tasks.insert(task_id, task.clone());
        self.sorted_tasks.insert(task.clone());
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let task = self.tasks.get_mut(&task_id).unwrap();

        self.sorted_tasks.remove(task);
        task.priority = new_priority;
        self.sorted_tasks.insert(task.clone());
    }

    fn rmv(&mut self, task_id: i32) {
        let task = self.tasks.remove(&task_id).unwrap();
        self.sorted_tasks.remove(&task);
    }

    fn exec_top(&mut self) -> i32 {
        if self.tasks.is_empty() {
            return -1;
        }
        let removed = self.sorted_tasks.pop_last().unwrap();
        self.tasks.remove(&removed.id);
        removed.user
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */

fn main() {
    println!("Hello, world!");
}
