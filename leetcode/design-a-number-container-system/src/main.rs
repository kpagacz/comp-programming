// https://leetcode.com/problems/design-a-number-container-system/description/
use std::collections::BTreeSet;
use std::collections::HashMap;
struct NumberContainers {
    num_to_indexes: HashMap<i32, BTreeSet<i32>>,
    index_to_num: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            num_to_indexes: HashMap::new(),
            index_to_num: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(old_num) = self.index_to_num.insert(index, number) {
            self.num_to_indexes.entry(old_num).and_modify(|set| {
                set.remove(&index);
            });
        }
        self.num_to_indexes.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        *self
            .num_to_indexes
            .get(&number)
            .and_then(|set| set.first())
            .unwrap_or(&-1)
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */

fn main() {
    println!("Hello, world!");
}
