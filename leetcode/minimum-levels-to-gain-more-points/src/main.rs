// https://leetcode.com/problems/minimum-levels-to-gain-more-points/description/
pub struct Solution;

impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let (ones, zeros) = possible.iter().fold((0, 0), |ones_zeros, &possible| {
            if possible == 1 {
                (ones_zeros.0 + 1, ones_zeros.1)
            } else {
                (ones_zeros.0, ones_zeros.1 + 1)
            }
        });

        let mut steps = 0;
        let mut score_left = ones - zeros;
        let mut alice_score = 0;
        if possible[0] == 1 {
            alice_score += 1;
            score_left -= 1;
        } else {
            alice_score -= 1;
            score_left += 1;
        }
        steps += 1;

        while alice_score <= score_left && steps < possible.len() {
            if possible[steps] == 1 {
                alice_score += 1;
                score_left -= 1;
            } else {
                alice_score -= 1;
                score_left += 1;
            }
            steps += 1;
        }

        if steps == possible.len() {
            -1
        } else {
            steps as _
        }
    }
}

fn main() {
    println!("Hello, world!");
}
