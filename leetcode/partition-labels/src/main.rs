// https://leetcode.com/problems/partition-labels/description/?envType=daily-question&envId=2025-03-30
pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut last = [0; 256];

        for (pos, c) in s.iter().enumerate() {
            last[*c as usize] = pos;
        }

        let mut partitions = vec![];
        let mut partition_start = 0;
        let mut partition_end = 0;

        for (pos, c) in s.iter().enumerate() {
            if pos > partition_end {
                partitions.push(partition_end as i32 - partition_start as i32 + 1);
                partition_start = pos;
            }
            partition_end = partition_end.max(last[*c as usize]);
        }
        partitions.push(partition_end as i32 - partition_start as i32 + 1);

        partitions
    }
}

fn main() {
    println!("Hello, world!");
}
