// https://leetcode.com/problems/number-of-people-aware-of-a-secret/description/?envType=daily-question&envId=2025-09-09
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let mut dp = vec![0; forget as usize];

        dp[0] = 1;
        for _ in 1..n {
            for i in (1..dp.len()).rev() {
                dp[i] = dp[i - 1];
            }
            dp[0] = dp[delay as usize..]
                .iter()
                .fold(0, |acc, count| (acc + count) % (10i32.pow(9) + 7));
        }

        dp.into_iter()
            .fold(0, |acc, count| (acc + count) % (10i32.pow(9) + 7))
    }
}

fn main() {
    let test_cases = [(6, 2, 4, 5)];

    for (n, delay, forget, exp) in test_cases {
        println!(
            "{}  exp: {exp}",
            Solution::people_aware_of_secret(n, delay, forget)
        );
    }
}
