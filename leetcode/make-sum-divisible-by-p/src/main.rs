// https://leetcode.com/problems/make-sum-divisible-by-p/description/
pub struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        use std::collections::HashMap;
        let mut prefix: HashMap<i32, usize> = HashMap::default();
        prefix.insert(0i32, 0);
        let target = nums.iter().fold(0, |acc, num| (acc + *num) % p);

        let mut to_remove = usize::MAX;
        let mut current_modulo = 0;
        if target == 0 {
            return 0;
        }
        for (pos, num) in nums.into_iter().enumerate() {
            current_modulo = (current_modulo + num) % p;
            // current_modulo - x = target
            // x = (current_modulo - target + p) % p
            let to_find = (current_modulo - target + p) % p;
            if let Some(&found_pos) = prefix.get(&to_find) {
                to_remove = to_remove.min(pos - found_pos + 1);
            }
            prefix.insert(current_modulo, pos + 1);
        }
        if to_remove == usize::MAX || to_remove == n {
            -1
        } else {
            to_remove as _
        }
    }
}

fn main() {
    println!("Hello, world!");
}
