// https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/description/
pub struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if m * k > bloom_day.len() as i32 {
            return -1;
        }

        let (mut left, mut right) = (0, 1_000_000_000);

        while left < right {
            let mid = left + (right - left) / 2;

            let mut bouqets = 0;
            let mut previously_smaller = 0;

            bloom_day.iter().for_each(|&days| {
                if days <= mid {
                    previously_smaller += 1;
                } else {
                    previously_smaller = 0;
                }
                if previously_smaller == k {
                    bouqets += 1;
                    previously_smaller = 0;
                }
            });

            if bouqets >= m {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

fn main() {
    println!("Hello, world!");
}
