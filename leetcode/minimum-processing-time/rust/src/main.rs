// https://leetcode.com/problems/minimum-processing-time/
pub struct Solution {}

impl Solution {
    pub fn min_processing_time(mut processor_time: Vec<i32>, mut tasks: Vec<i32>) -> i32 {
        tasks.sort_unstable();
        processor_time.sort_unstable();

        let mut max_time = i32::MIN;
        processor_time.iter().for_each(|time| {
            max_time = max_time.max(tasks[tasks.len() - 1] + time);
            for _ in 0..4 {
                tasks.pop();
            }
        });
        max_time
    }
}

fn main() {
    println!("Hello, world!");
}
