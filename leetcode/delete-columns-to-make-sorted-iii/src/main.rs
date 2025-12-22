// https://leetcode.com/problems/delete-columns-to-make-sorted-iii/description/?envType=daily-question&envId=2025-12-22
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut dp = vec![1; strs[0].len()];
        let n = dp.len();
        for i in (0..=n.saturating_sub(2)).rev() {
            'search: for j in i + 1..n {
                for str in &strs {
                    if str.as_bytes()[i] > str.as_bytes()[j] {
                        continue 'search;
                    }
                }
                dp[i] = dp[i].max(1 + dp[j]);
            }
        }
        n as i32 - dp.into_iter().fold(0, |acc, kept| acc.max(kept))
    }
}

fn main() {
    println!("Hello, world!");
}
