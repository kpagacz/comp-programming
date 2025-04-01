// https://leetcode.com/problems/put-marbles-in-bags/description/?envType=daily-question&envId=2025-03-31
pub struct Solution;

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut min_pairs: Vec<i32> = weights
            .windows(2)
            .map(|window| window[0] + window[1])
            .collect();
        min_pairs.sort_unstable();
        let min_score_distribution = min_pairs
            .into_iter()
            .take(k as usize - 1)
            .map(|sum| sum as i64)
            .sum::<i64>();

        let mut max_pairs: Vec<i32> = weights
            .windows(2)
            .map(|window| window[0] + window[1])
            .collect();
        max_pairs.sort_unstable_by_key(|sum| std::cmp::Reverse(*sum));
        let max_score_distribution = max_pairs
            .into_iter()
            .take(k as usize - 1)
            .map(|sum| sum as i64)
            .sum::<i64>();
        max_score_distribution - min_score_distribution
    }
}

fn main() {
    println!("Hello, world!");
}
