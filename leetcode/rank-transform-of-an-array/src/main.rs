// https://leetcode.com/problems/rank-transform-of-an-array/description/
pub struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted: Vec<_> = (0..arr.len()).collect();
        sorted.sort_unstable_by_key(|&id| arr[id]);

        let mut rank = 1;
        let mut ranks = vec![1; arr.len()];

        for id_pos in 1..sorted.len() {
            if arr[sorted[id_pos]] != arr[sorted[id_pos - 1]] {
                rank += 1;
            }

            ranks[sorted[id_pos]] = rank;
        }

        ranks
    }
}

fn main() {
    println!("Hello, world!");
}
