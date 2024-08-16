// https://leetcode.com/problems/maximum-distance-in-arrays/description/
pub struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut maxes = BinaryHeap::from_iter(
            arrays
                .iter()
                .enumerate()
                .map(|(pos, array)| (array[array.len() - 1], pos)),
        );
        let mut mins = BinaryHeap::from_iter(
            arrays
                .iter()
                .enumerate()
                .map(|(pos, array)| (Reverse(array[0]), pos)),
        );

        let max = maxes.pop().unwrap();
        let min = mins.pop().unwrap();

        if max.1 == min.1 {
            let mut max_diff = 0;
            while let Some(next_min) = mins.pop() {
                if next_min.1 == max.1 {
                    continue;
                }

                max_diff = max_diff.max(max.0 - next_min.0 .0);
            }

            while let Some(next_max) = maxes.pop() {
                if next_max.1 == min.1 {
                    continue;
                }

                max_diff = max_diff.max(next_max.0 - min.0 .0);
            }

            max_diff
        } else {
            max.0 - min.0 .0
        }
    }
}

fn main() {
    println!("Hello, world!");
}
