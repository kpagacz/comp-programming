// https://leetcode.com/problems/longest-subsequence-with-limited-sum/
pub struct Solution;

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut prefixes = nums;
        prefixes.sort_unstable();
        prefixes.splice(0..0, std::iter::once(0));
        for i in 1..prefixes.len() {
            prefixes[i] += prefixes[i - 1];
        }

        queries
            .into_iter()
            .map(|query| prefixes.partition_point(|prefix| prefix <= &query) as i32 - 1)
            .collect()
    }
}

fn main() {
    let test_cases = [(vec![4, 5, 2, 1], vec![3, 10, 21]), (vec![7], vec![1])];
    for (nums, queries) in test_cases {
        println!("{:?}", Solution::answer_queries(nums, queries));
    }
}
