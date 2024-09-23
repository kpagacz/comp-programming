// https://leetcode.com/problems/extra-characters-in-a-string/description/
pub struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut dp = vec![0; s.len() + 1];
        for start in (0..s.len()).rev() {
            dp[start] = dp[start + 1] + 1;
            for word in &dictionary {
                if s[start..].starts_with(word) {
                    dp[start] = dp[start].min(dp[start + word.len()]);
                }
            }
        }
        dp[0]
    }
}

fn main() {
    let test_cases = [("leetscode", vec!["leet", "code", "leetcode"])];

    for (s, dictionary) in test_cases {
        println!(
            "{}",
            Solution::min_extra_char(
                s.to_owned(),
                dictionary.into_iter().map(str::to_owned).collect()
            )
        );
    }
}
