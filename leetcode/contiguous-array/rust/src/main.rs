// https://leetcode.com/problems/contiguous-array/description/
pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut prefixes = HashMap::new();
        prefixes.insert(0, 0);
        let mut prefix = 0;
        let mut max_length = 0;
        for (id, num) in nums.iter().enumerate() {
            if num == &0 {
                prefix -= 1;
            } else {
                prefix += 1;
            }
            if let Some(previous_id) = prefixes.get(&prefix) {
                max_length = max_length.max(id - previous_id + 1);
            } else {
                prefixes.insert(prefix, id + 1);
            }
        }

        max_length as _
    }
}

fn main() {
    println!("Hello, world!");
}
