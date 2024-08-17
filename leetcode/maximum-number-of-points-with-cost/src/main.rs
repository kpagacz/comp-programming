// https://leetcode.com/problems/maximum-number-of-points-with-cost/description/
pub struct Solution;

impl Solution {
    pub fn max_points(mut points: Vec<Vec<i32>>) -> i64 {
        let mut dp = Vec::from_iter(points[0].iter().map(|&num| num as i64));
        let mut max_gain = dp.clone();

        points[1..].iter_mut().for_each(|row| {
            let mut suffix_max = *max_gain.last().unwrap();
            let mut prefix_max = *max_gain.first().unwrap();

            for gain in max_gain.iter_mut() {
                prefix_max = prefix_max.max(*gain);
                *gain = prefix_max;
                prefix_max -= 1;
            }

            for gain in max_gain.iter_mut().rev() {
                suffix_max = suffix_max.max(*gain);
                *gain = suffix_max;
                suffix_max -= 1;
            }

            for i in 0..max_gain.len() {
                dp[i] = row[i] as i64 + max_gain[i];
            }
            max_gain.copy_from_slice(&dp[..]);
        });

        dp.into_iter().max().unwrap()
    }
}

fn main() {
    let test_cases = [vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]]];

    for points in test_cases {
        println!("{}", Solution::max_points(points));
    }
}
