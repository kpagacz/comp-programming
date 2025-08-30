//  https://leetcode.com/problems/alice-and-bob-playing-flower-game/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        // The sum of flowers in both lanes must be odd for the first player to win
        // odd + even, even + odd
        let first_lane_even = n as i64 / 2;
        let first_lane_odd = (n as i64 + 1) / 2;
        let second_lane_even = m as i64 / 2;
        let second_lane_odd = (m as i64 + 1) / 2;

        first_lane_odd * second_lane_even + first_lane_even * second_lane_odd
    }
}

fn main() {
    println!("Hello, world!");
}
