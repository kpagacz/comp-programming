// https://leetcode.com/problems/student-attendance-record-ii/description/
pub struct Solution;

// Optimizations:
// * only need the previous day, so only need 6 variables
const MOD: i32 = 10i32.pow(9) + 7;
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let max_abs = 2;
        let max_late = 3;
        let mut dp = vec![vec![vec![0; max_abs]; max_late]; n as usize + 1];

        dp[0][0][0] = 1;

        for day in 1..=n as usize {
            for cons_late in 0..max_late {
                for abs in 0..max_abs {
                    if cons_late > 0 {
                        dp[day][cons_late][abs] =
                            (dp[day][cons_late][abs] + dp[day - 1][cons_late - 1][abs]) % MOD;
                    } else {
                        for l in 0..max_late {
                            dp[day][cons_late][abs] =
                                (dp[day][cons_late][abs] + dp[day - 1][l][abs]) % MOD;
                        }
                        if abs > 0 {
                            for l in 0..max_late {
                                dp[day][cons_late][abs] =
                                    (dp[day][cons_late][abs] + dp[day - 1][l][abs - 1]) % MOD;
                            }
                        }
                    }
                }
            }
        }

        let mut ans = 0;
        for i in 0..max_abs {
            for j in 0..max_late {
                ans = (ans + dp[n as usize][j][i]) % MOD;
            }
        }
        ans
    }
}

fn main() {
    let test_cases = [2, 1, 10101];
    for n in test_cases {
        println!("{}", Solution::check_record(n));
    }
}
