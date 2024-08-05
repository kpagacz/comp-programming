// https://leetcode.com/problems/kth-distinct-string-in-an-array/description/
pub struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        use std::collections::HashMap;

        let mut freq: HashMap<&String, i32> = HashMap::new();
        for word in &arr {
            *freq.entry(word).or_default() += 1;
        }

        let mut distinct_instances = 0;

        for word in &arr {
            if *freq.get(word).unwrap_or(&0) == 1 {
                distinct_instances += 1;
            }

            if distinct_instances == k {
                return word.clone();
            }
        }

        "".to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
