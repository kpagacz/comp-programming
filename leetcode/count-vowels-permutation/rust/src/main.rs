// https://leetcode.com/problems/count-vowels-permutation/description/
pub struct Solution {}

const MOD: i32 = 10i32.pow(9) + 7;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let allowed_chars = 5usize;
        let n = n as usize;
        let mut dp = vec![vec![0; allowed_chars]; n];
        (0..allowed_chars).for_each(|id| dp[0][id] = 1); // base cases
        (1..dp.len()).for_each(|remaining| {
            dp[remaining][0] = dp[remaining - 1][1];
            dp[remaining][1] = (dp[remaining - 1][0] + dp[remaining - 1][2]) % MOD;
            dp[remaining][2] = [
                dp[remaining - 1][0],
                dp[remaining - 1][1],
                dp[remaining - 1][3],
                dp[remaining - 1][4],
            ]
            .into_iter()
            .fold(0, |acc, count| (acc + count) % MOD);
            dp[remaining][3] = (dp[remaining - 1][2] + dp[remaining - 1][4]) % MOD;
            dp[remaining][4] = dp[remaining - 1][0];
        });

        // println!("{dp:?}");
        (0..allowed_chars)
            .map(|id| dp[n - 1][id])
            .fold(0, |acc, count| (acc + count) % MOD)
    }
    pub fn count_vowel_permutation_space_opt(n: i32) -> i32 {
        const MODULO: u32 = 10u32.pow(9) + 7;
        ((1..n)
            .fold([1; 5], |previous, _| {
                [
                    previous[1],
                    (previous[0] + previous[2]) % MODULO,
                    (previous[0] + previous[1] + previous[3] + previous[4]) % MODULO,
                    (previous[2] + previous[4]) % MODULO,
                    previous[0],
                ]
            })
            .iter()
            .sum::<u32>()
            % MODULO) as i32
    }
}

fn main() {
    let test_cases = vec![1];
    for n in test_cases {
        println!("{}", Solution::count_vowel_permutation(n));
    }
}
