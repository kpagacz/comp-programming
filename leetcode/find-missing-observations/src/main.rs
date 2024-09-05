// https://leetcode.com/problems/find-missing-observations/description/
pub struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let current_sum = rolls.iter().sum::<i32>();
        let target_sum = mean * (n + rolls.len() as i32);
        let mut remaining = target_sum - current_sum;

        if remaining < 1 || remaining > n * 6 {
            return Vec::new();
        }

        let mut answer = vec![1; n as usize];
        remaining -= n;

        answer.iter_mut().for_each(|roll| {
            if remaining > 0 {
                let to_add = 5.min(remaining);
                remaining -= to_add;
                *roll += to_add;
            }
        });
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
