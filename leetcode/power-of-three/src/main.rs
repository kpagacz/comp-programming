// https://leetcode.com/problems/power-of-three/?envType=daily-question&envId=2025-08-13
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let max_power_of_three = 1162261467;
        n > 0 && max_power_of_three % n == 0
    }
}

fn main() {
    println!("Hello, world!");
}
