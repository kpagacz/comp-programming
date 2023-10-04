// https://leetcode.com/problems/design-hashmap/description/
struct MyHashMap {
    values: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        Self {
            values: vec![-1; 1_000_001],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.values[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.values[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.values[key as usize] = -1;
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

fn main() {
    println!("Hello, world!");
}
