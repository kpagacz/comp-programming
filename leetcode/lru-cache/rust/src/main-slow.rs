// https://leetcode.com/problems/lru-cache/

// Runtime
// Details
// 439ms
// Beats 6.35%of users with Rust
// Memory
// Details
// 103.72mb
// Beats 77.78%of users with Rust
struct LRUCache {
    capacity: i32,
    age: i32,
    cache: std::collections::HashMap<i32, (i32, i32)>, // value, age
    oldest_key: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            age: 0,
            cache: std::collections::HashMap::with_capacity(capacity as usize),
            oldest_key: -1,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.cache.contains_key(&key) {
            let old_age = self.age.clone();
            self.cache.entry(key).and_modify(|(_, age)| {
                *age = old_age;
            });
            self.age += 1;
            if key == self.oldest_key {
                self.oldest_key = Self::find_oldest(&self);
            }
            self.cache.get(&key).unwrap().0
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let old_age = self.age.clone();
        if self.cache.contains_key(&key) || (self.cache.len() as i32) < self.capacity {
            self.cache
                .entry(key)
                .and_modify(|(cached_value, age)| {
                    *cached_value = value;
                    *age = old_age;
                })
                .or_insert((value, self.age));
            if key == self.oldest_key || self.oldest_key == -1 {
                self.oldest_key = Self::find_oldest(&self);
            }
        } else {
            Self::evict_oldest(self);
            self.cache.insert(key, (value, self.age));
        }
        self.age += 1;
    }

    fn evict_oldest(&mut self) {
        self.cache.remove(&self.oldest_key);
        self.oldest_key = Self::find_oldest(&self);
    }

    fn find_oldest(&self) -> i32 {
        *self
            .cache
            .iter()
            .min_by_key(|entry| entry.1 .1)
            .unwrap_or_else(|| (&-1, &(-1, -1)))
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
