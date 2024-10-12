// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/description
pub struct Solution;

impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        intervals.sort_unstable();
        let mut ends = BinaryHeap::default();
        let mut max_size = 0;
        for interval in intervals {
            let (start, end) = (interval[0], interval[1]);
            while let Some(Reverse(earliest_end)) = ends.peek().copied() {
                if earliest_end < start {
                    ends.pop();
                } else {
                    break;
                }
            }
            ends.push(Reverse(end));
            max_size = max_size.max(ends.len());
        }
        max_size as _
    }
}

fn main() {
    println!("Hello, world!");
}
