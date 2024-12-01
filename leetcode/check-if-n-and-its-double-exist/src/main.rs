// https://leetcode.com/problems/check-if-n-and-its-double-exist/description/?envType=daily-question&envId=2024-12-01
pub struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut seen = HashSet::new();
        for num in arr {
            if seen.contains(&(num * 2)) || (num % 2 == 0 && seen.contains(&(num / 2))) {
                return true;
            } else {
                seen.insert(num);
            }
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
