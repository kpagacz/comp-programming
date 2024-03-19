// https://leetcode.com/problems/task-scheduler/description/
pub struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        use std::collections::HashMap;
        let freqs = tasks.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        let max_freq = *freqs.values().max().unwrap();
        let max_freq_count = freqs.values().filter(|&&v| v == max_freq).count() as i32;

        let result = (max_freq - 1) * (n + 1) + max_freq_count;
        result.max(tasks.len() as _)
    }
}

fn main() {
    println!("Hello, world!");
}
