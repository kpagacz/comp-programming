// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-empty/
pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut frequencies = nums.into_iter().fold(HashMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

        let mut answer = 0;
        for &frequency in frequencies.values() {
            if frequency == 1 {
                return -1;
            }
            answer += match frequency % 3 {
                0 => frequency / 3,
                1 | 2 => frequency / 3 + 1,
                _ => unreachable!(),
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
