// https://leetcode.com/problems/lru-cache/

// Runtime
// Details
// 195ms
// Beats 34.92%of users with Rust
// Memory
// Details
// 103.78mb
// Beats 77.78%of users with Rust
struct LRUCache {
    capacity: i32,
    current_size: i32,
    age: i32,
    cache: Vec<(i32, i32)>, // value, age
    oldest_key: Option<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            current_size: 0,
            age: 0,
            cache: vec![(-1, 200001); 4001],
            oldest_key: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if key > 4000 {
            return -1;
        }
        let usize_key = key as usize;
        if self.cache[usize_key].0 != -1 {
            self.cache[usize_key].1 = self.age;
            self.age += 1;
        }
        if self.oldest_key == Some(usize_key) {
            self.oldest_key = Some(Self::find_oldest(&self));
        }
        self.cache[usize_key].0
    }

    fn put(&mut self, key: i32, value: i32) {
        let usize_key = key as usize;
        if self.cache[usize_key].0 != -1 || self.current_size < self.capacity {
            if self.cache[usize_key].0 == -1 {
                self.current_size += 1;
            }
            self.cache[usize_key] = (value, self.age);
            if self.oldest_key.is_none() || self.oldest_key.unwrap() == usize_key {
                self.oldest_key = Some(Self::find_oldest(&self));
            }
        } else {
            Self::evict_oldest(self);
            self.cache[usize_key] = (value, self.age);
        }
        self.age += 1;
    }

    fn evict_oldest(&mut self) {
        self.cache[self.oldest_key.unwrap()] = (-1, 200001);
        self.oldest_key = Some(Self::find_oldest(&self));
    }

    fn find_oldest(&self) -> usize {
        self.cache
            .iter()
            .enumerate()
            .min_by_key(|(_, &entry)| entry.1)
            .unwrap()
            .0
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
