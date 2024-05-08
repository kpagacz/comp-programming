// https://leetcode.com/problems/teemo-attacking/description/
pub struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut last_attack = i32::MIN;
        let mut answer = 0;

        for time in time_series {
            answer += duration - (last_attack + duration).saturating_sub(time).max(0);
            last_attack = time;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
