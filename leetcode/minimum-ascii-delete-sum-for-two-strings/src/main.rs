// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in 0..dp.len() {
            for j in 0..dp[0].len() {
                let mut res = if i == 0 && j == 0 { 0 } else { i32::MAX };
                if i > 0 && j > 0 && s1[i - 1] == s2[j - 1] {
                    res = dp[i - 1][j - 1];
                } else if i > 0 && j > 0 && s1[i - 1] != s2[j - 1] {
                    res = res.min(dp[i - 1][j - 1] + s1[i - 1] as i32 + s2[j - 1] as i32);
                }
                if i > 0 {
                    res = res.min(dp[i - 1][j] + s1[i - 1] as i32);
                }
                if j > 0 {
                    res = res.min(dp[i][j - 1] + s2[j - 1] as i32);
                }
                dp[i][j] = res;
            }
        }

        dp[s1.len()][s2.len()]
    }
}

fn main() {
    let test_cases = [("sea", "eat", 231)];

    for (s1, s2, exp) in test_cases {
        let s1 = s1.to_string();
        let s2 = s2.to_string();
        println!("{} exp {exp}", Solution::minimum_delete_sum(s1, s2));
    }
}
