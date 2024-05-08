// https://leetcode.com/problems/relative-ranks/description/
pub struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        use std::collections::BinaryHeap;
        let n = score.len();
        let mut scores =
            score
                .into_iter()
                .enumerate()
                .fold(BinaryHeap::new(), |mut heap, (pos, score)| {
                    heap.push((score, pos));
                    heap
                });

        let mut answer = vec![String::default(); n];

        let mut position = 0;
        while let Some((_, index)) = scores.pop() {
            position += 1;
            match position {
                1 => answer[index] = "Gold Medal".to_string(),
                2 => answer[index] = "Silver Medal".to_string(),
                3 => answer[index] = "Bronze Medal".to_string(),
                _ => answer[index] = position.to_string(),
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
