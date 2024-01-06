// https://leetcode.com/problems/maximum-profit-in-job-scheduling/description/
pub struct Solution;

#[derive(Debug)]
struct Job(i32, i32, i32);
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time
            .iter()
            .zip(end_time)
            .zip(profit)
            .map(|((start, end), profit)| Job(*start, end, profit))
            .collect::<Vec<_>>();
        jobs.sort_unstable_by_key(|job| (job.0, job.1));

        let mut dp = vec![-1; start_time.len() + 1];
        *dp.last_mut().unwrap() = 0;

        fn max_profit(pos: usize, jobs: &[Job], dp: &mut [i32]) -> i32 {
            if dp[pos] != -1 {
                return dp[pos];
            }
            dp[pos] = dp[pos].max(max_profit(pos + 1, jobs, dp));
            let index_of_pos_end = jobs.partition_point(|job| job.0 < jobs[pos].1);
            dp[pos] = dp[pos].max(jobs[pos].2 + max_profit(index_of_pos_end, jobs, dp));
            dp[pos]
        }
        max_profit(0, &jobs, &mut dp);

        dp[0]
    }
}

fn main() {
    let test_cases = [
        (vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
        (
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60],
        ),
        (vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
    ];

    for (start_time, end_time, profit) in test_cases {
        println!("{}", Solution::job_scheduling(start_time, end_time, profit));
    }
}
