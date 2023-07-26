// https://leetcode.com/problems/minimum-speed-to-arrive-on-time/
pub struct Solution {}
impl Solution {
    // Runtime 68ms Beats 75.00% of users with Rust
    // Memory 3.87mb Beats 25.00% of users with Rust
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let (mut left, mut right) = (1, 10000000);
        if Self::calculate_time(&dist, right as f64) > hour {
            return -1;
        }
        while left < right {
            let middle = (left + right) / 2;
            if Solution::calculate_time(&dist, middle as f64) <= hour {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        left
    }

    fn calculate_time(dist: &Vec<i32>, speed: f64) -> f64 {
        let without_last: f64 = dist
            .iter()
            .take(dist.len() - 1)
            .map(|&d| (d as f64 / speed).ceil())
            .sum();
        without_last + *dist.last().unwrap() as f64 / speed
    }
}
fn main() {
    println!("Hello, world!");
}
