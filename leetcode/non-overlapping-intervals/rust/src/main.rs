// https://leetcode.com/problems/non-overlapping-intervals/

pub struct Solution {}

impl Solution {
    // Runtime 60ms
    // Beats 84.21%of users with Rust
    // Memory 9.08mb
    // Beats 94.74%of users with Rust
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable();
        let mut removed = 0;
        let mut current_interval = (std::i32::MIN, std::i32::MIN);

        intervals.iter().for_each(|interval| {
            let (begin, end) = (interval[0], interval[1]);
            if begin >= current_interval.1 {
                current_interval = (begin, end);
                return;
            }
            if current_interval.1 > end {
                current_interval = (begin, end);
            }
            removed += 1;
        });

        removed
    }

    // Runtime 46ms
    // Beats 92.11%of users with Rust
    // Memory 10.26mb
    // Beats 10.53%of users with Rust
    // It turns out you don't actually need the starts, you only need to sort
    // by the ends end then rely on that sorting to determine which interval to drop
    pub fn erase_overlap_intervals_faster(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|interval| interval[1]);
        let mut removed = 0;
        let mut last_end = std::i32::MIN;

        intervals.iter().for_each(|interval| {
            if interval[0] < last_end {
                removed += 1;
            } else {
                last_end = interval[1];
            }
        });

        removed
    }
}

fn main() {
    let tests: Vec<Vec<Vec<i32>>> = vec![
        vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]],
        vec![vec![1, 2], vec![1, 2], vec![1, 2]],
        vec![vec![1, 2], vec![2, 3]],
    ];

    for test in tests {
        println!("{}", Solution::erase_overlap_intervals(test));
    }
}
