// https://leetcode.com/problems/champagne-tower/description/?envType=daily-question&envId=2026-02-14
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = [0f64; 101];
        dp[0] = poured as f64;

        for _ in 0..query_row {
            let mut new_dp = [0f64; 101];
            for i in 0..dp.len() {
                if i == 0 {
                    new_dp[i] = (dp[0] - 1.0).max(0f64) / 2.0;
                } else {
                    new_dp[i] = (dp[i - 1] - 1.0).max(0f64) / 2.0 + (dp[i] - 1.0).max(0f64) / 2.0;
                }
            }
            dp = new_dp;
        }

        dp[query_glass as usize].min(1f64)
    }
}

fn main() {
    let test_cases = [(1, 1, 1, 0.0), (2, 1, 1, 0.5), (100000009, 33, 17, 1.0)];

    for (poured, query_row, query_glass, exp) in test_cases {
        println!(
            "{} exp {exp}",
            Solution::champagne_tower(poured, query_row, query_glass)
        );
    }
}
