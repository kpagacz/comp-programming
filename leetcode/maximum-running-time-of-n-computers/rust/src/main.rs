// https://leetcode.com/problems/maximum-running-time-of-n-computers/
struct Solution {}

#[allow(dead_code)]
impl Solution {
    // Turns out you don't even have to sort this array. Just taking a min from target and the actual
    // value of the battery time and summing over there is enough. Then you compare the sum with
    // n * target and if it's larger or equal, the time is possible.
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let (mut left, mut right) = (0_i64, 100_000 * 1_000_000_000 + 1);

        while left < right {
            let middle = (left + right) / 2;

            if Self::can_run_simultaneously(&batteries, middle, n) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left - 1
    }

    fn can_run_simultaneously(batteries: &[i32], time: i64, computers: i32) -> bool {
        batteries
            .iter()
            .map(|battery| (*battery as i64).min(time))
            .sum::<i64>()
            >= (computers as i64) * time
    }
}
fn main() {
    println!("Hello, world!");
}
