// https://leetcode.com/problems/bag-of-tokens/description/
pub struct Solution;

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let mut left = 0usize;
        let mut right = tokens.len() - 1;
        tokens.sort_unstable();

        let mut score = 0;
        let mut max_score = 0;

        while left < right {
            if tokens[left] <= power {
                score += 1;
                max_score = max_score.max(score);
                power -= tokens[left];
                left += 1;
            } else if score > 0 {
                score -= 1;
                power += tokens[right];
                right -= 1;
            } else {
                break;
            }
        }

        if power >= tokens[left] {
            score += 1;
            max_score = max_score.max(score);
        }

        max_score
    }
}

fn main() {
    println!("Hello, world!");
}
