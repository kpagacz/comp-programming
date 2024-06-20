// https://leetcode.com/problems/magnetic-force-between-two-balls/description/
pub struct Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();
        let (mut left, mut right) = (1, 1_000_000_000);

        while left < right {
            let mid = left + (right - left) / 2;

            let (mut ms, mut previous) = (1, position[0]);
            for i in 1..position.len() {
                if position[i] >= previous + mid {
                    ms += 1;
                    previous = position[i];
                }
            }

            if ms >= m {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left - 1
    }
}

fn main() {
    println!("Hello, world!");
}
