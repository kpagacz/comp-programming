// https://leetcode.com/problems/rotate-string/description/
pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let s = format!("{s}{s}");
        Solution::kmp(s, goal)
    }

    fn kmp(s: String, pattern: String) -> bool {
        let concatenated = format!("{pattern}${s}");
        let concatenated = concatenated.as_bytes();
        let mut dp = vec![0; concatenated.len()];
        for i in 1..dp.len() {
            let mut previous = dp[i - 1];

            while previous > 0 && concatenated[previous] != concatenated[i] {
                previous = dp[previous - 1];
            }
            if concatenated[previous] == concatenated[i] {
                previous += 1;
            }
            dp[i] = previous;
        }

        dp.iter().any(|matched| *matched == pattern.len())
    }
}

fn main() {
    let test_cases = [
        ("abcd", "e"),
        ("something", "ethi"),
        ("lala", "lal"),
        ("lala", "lala"),
    ];

    for (s, pattern) in test_cases {
        println!(
            "{pattern} in {s}: {}",
            Solution::kmp(s.to_owned(), pattern.to_owned())
        );
    }
}
