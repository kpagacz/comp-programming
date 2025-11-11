// https://leetcode.com/problems/ones-and-zeroes/description/?envType=daily-question&envId=2025-11-11
struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![vec![i32::MIN; n + 1]; m + 1]; strs.len() + 1];
        // dp[i][m][n] = the max subset size that contains
        // i strings and contains m 0s and n 1s.
        dp[0][0][0] = 0;
        for i in 0..strs.len() {
            let zeros = strs[i].as_bytes().iter().filter(|b| **b == b'0').count();
            let ones = strs[i].as_bytes().iter().filter(|b| **b == b'1').count();

            for it_zeros in 0..=m {
                for it_ones in 0..=n {
                    dp[i + 1][it_zeros][it_ones] = dp[i][it_zeros][it_ones];
                    if it_zeros >= zeros && it_ones >= ones {
                        dp[i + 1][it_zeros][it_ones] = dp[i + 1][it_zeros][it_ones]
                            .max(dp[i][it_zeros - zeros][it_ones - ones] + 1);
                    }
                }
            }
        }

        dp.into_iter()
            .map(|m2d| {
                m2d.into_iter()
                    .map(|row| row.into_iter().max().unwrap())
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let test_cases = [
        (vec!["10", "0001", "111001", "1", "0"], 5, 3),
        (vec!["10", "0", "1"], 1, 1),
        (vec!["10", "0001", "111001", "1", "0"], 4, 3),
    ];
    for (strs, m, n) in test_cases {
        let strs = strs.into_iter().map(str::to_owned).collect();
        println!("{}", Solution::find_max_form(strs, m, n));
    }
}
