// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/description/?envType=daily-question&envId=2025-06-28
pub struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums_with_index: Vec<_> = nums
            .iter()
            .enumerate()
            .map(|(pos, num)| (num, pos))
            .collect();
        nums_with_index.sort_unstable();
        let mut top_k_nums: Vec<_> = nums_with_index
            .into_iter()
            .rev()
            .take(k as usize)
            .map(|(_, pos)| pos)
            .collect();
        top_k_nums.sort_unstable();
        top_k_nums.into_iter().map(|pos| nums[pos]).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
