// https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
pub struct Solution {}

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;
        let (n, m, k) = (n as usize, m as usize, k as usize);
        let mut dp = vec![vec![vec![0; k + 1]; m + 1]; n + 1];
        (0..dp[0].len()).for_each(|max| dp[n][max][0] = 1);

        let mut logic = |a: usize, b: usize, c: usize| {
            let mut ans = (b as i64 * dp[a + 1][b][c]) % MOD;
            if c > 0 {
                (b + 1..m + 1).for_each(|max| ans = (ans + dp[a + 1][max][c - 1]) % MOD);
            }
            dp[a][b][c] = ans;
        };

        (0..n).rev().for_each(|a| {
            (0..m + 1).rev().for_each(|b| {
                (0..k + 1).for_each(|c| logic(a, b, c));
            })
        });

        dp[0][0][k] as _
    }
}

fn main() {
    println!("{}", Solution::num_of_arrays(5, 10, 2));
}
