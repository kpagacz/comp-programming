// https://leetcode.com/problems/lru-cache/

use std::collections::{HashMap, VecDeque};

// Runtime
// Details
// 139ms
// Beats 73.02%of users with Rust
// Memory
// Details
// 103.89mb
// Beats 53.97%of users with Rust
struct LRUCache {
    capacity: i32,
    values: HashMap<i32, i32>,
    refs: HashMap<i32, i32>,
    accesses: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            values: HashMap::with_capacity(capacity as usize),
            refs: HashMap::with_capacity(capacity as usize),
            accesses: VecDeque::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.values.get(&key) {
            self.refs.entry(key).and_modify(|rc| *rc += 1);
            self.accesses.push_back(key);
            *value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(cached) = self.values.get_mut(&key) {
            *cached = value;
            self.refs.entry(key).and_modify(|rc| *rc += 1);
            self.accesses.push_back(key);
        } else {
            while self.values.len() >= self.capacity as usize {
                let accessed_key = self.accesses.pop_front().unwrap();
                let times_accessed = self.refs.get_mut(&accessed_key).unwrap();
                if *times_accessed > 1 {
                    *times_accessed -= 1;
                } else {
                    self.values.remove(&accessed_key);
                    self.refs.remove(&accessed_key);
                    break;
                }
            }
            self.values.insert(key, value);
            self.refs.insert(key, 1);
            self.accesses.push_back(key);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {
    println!("Hello, world!");
}
