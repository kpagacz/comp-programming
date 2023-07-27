// https://leetcode.com/problems/maximum-running-time-of-n-computers/
pub struct Solution {}
impl Solution {
    // Turns out you don't even have to sort this array. Just taking a min from target and the actual
    // value of the battery time and summing over there is enough. Then you compare the sum with
    // n * target and if it's larger or equal, the time is possible.
    pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
        batteries.sort_unstable();
        batteries.reverse();

        let (mut left, mut right) = (0_i64, 1_00_000 * 1_000_000_000 + 1);

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

    fn can_run_simultaneously(sorted_batteries: &Vec<i32>, time: i64, computers: i32) -> bool {
        sorted_batteries
            .iter()
            .skip(computers as usize)
            .map(|&el| el as i64)
            .sum::<i64>()
            >= sorted_batteries
                .iter()
                .take(computers as usize)
                .filter_map(|&battery_time| {
                    if (battery_time as i64) < time {
                        Some(time - (battery_time as i64))
                    } else {
                        None
                    }
                })
                .sum::<i64>()
    }
}
fn main() {
    println!("Hello, world!");
}
