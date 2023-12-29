// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/
pub struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let jobs = job_difficulty.len();
        let mut dp = vec![vec![i32::MAX; jobs + 1]; d + 1];
        dp[0][0] = 0;

        for day in 1..=d {
            for job in 1..=jobs {
                let mut max_difficulty = i32::MIN;
                for additional_job in job..dp[0].len() {
                    max_difficulty = max_difficulty.max(job_difficulty[additional_job - 1]);
                    dp[day][additional_job] = dp[day][additional_job]
                        .min(max_difficulty.saturating_add(dp[day - 1][job - 1]));
                }
            }
        }
        if dp[d][jobs] == i32::MAX {
            -1
        } else {
            dp[d][jobs]
        }
    }
}

fn main() {
    let test_cases = [
        (vec![6, 5, 4, 3, 2, 1], 2),
        (vec![9, 9, 9], 4),
        (vec![1, 1, 1], 3),
    ];
    for (job_difficulty, d) in test_cases {
        println!("{}", Solution::min_difficulty(job_difficulty, d));
    }
}
