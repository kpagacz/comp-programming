// https://leetcode.com/problems/minimum-total-distance-traveled/description/
pub struct Solution;

impl Solution {
    pub fn minimum_total_distance(mut robots: Vec<i32>, mut factories: Vec<Vec<i32>>) -> i64 {
        robots.sort_unstable();
        factories.sort_unstable_by_key(|factory| factory[0]);
        let factories = factories
            .into_iter()
            .flat_map(|factory| vec![factory[0]; factory[1] as usize])
            .collect::<Vec<_>>();

        let mut dp = vec![vec![0; factories.len() + 1]; robots.len() + 1];

        for robot in (0..robots.len()).rev() {
            *dp[robot].last_mut().unwrap() = i64::MAX;
            for factory in (0..factories.len()).rev() {
                let use_factory = ((robots[robot] - factories[factory]).abs() as i64)
                    .saturating_add(dp[robot + 1][factory + 1]);
                let skip = dp[robot][factory + 1];
                dp[robot][factory] = i64::min(use_factory, skip);
            }
        }

        dp[0][0]
    }
}

fn main() {
    let test_cases = [(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]])];

    for (robots, factories) in test_cases {
        println!("{}", Solution::minimum_total_distance(robots, factories));
    }
}
