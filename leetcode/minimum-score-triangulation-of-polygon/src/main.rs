// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/description/?envType=daily-question&envId=2025-09-29
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![0; n + 2]; n + 2];

        fn rec(start: usize, stop: usize, dp: &mut [Vec<i32>], values: &[i32]) -> i32 {
            if start + 2 > stop {
                return 0;
            }
            if stop == start + 2 {
                return values[start] * values[start + 1] * values[start + 2];
            }
            if dp[start][stop] != 0 {
                return dp[start][stop];
            }
            let mut min_score = i32::MAX;
            for third in start + 1..stop {
                let triangle = values[start] * values[third] * values[stop];
                let other = rec(start, third, dp, values) + rec(third, stop, dp, values);
                min_score = min_score.min(triangle + other);
            }
            dp[start][stop] = min_score;
            dp[start][stop]
        }

        rec(0, n - 1, &mut dp, &values);

        dp[0][n - 1]
    }

    pub fn bottom_up(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![0; values.len()]; values.len()];
        for start in (0..n).rev() {
            for end in start + 2..n {
                let mut min_triangulation = i32::MAX;
                for middle in start + 1..end {
                    min_triangulation = min_triangulation.min(
                        values[start] * values[middle] * values[end]
                            + dp[start][middle]
                            + dp[middle][end],
                    );
                }
                dp[start][end] = min_triangulation;
            }
        }
        dp[0][n - 1]
    }
}

fn main() {
    println!("Hello, world!");
}
