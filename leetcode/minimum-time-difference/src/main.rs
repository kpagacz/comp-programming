// https://leetcode.com/problems/minimum-time-difference/description/
pub struct Solution;

#[derive(Debug, Clone, PartialOrd, Ord, Eq, PartialEq)]
struct Time(i32); // minutes

impl From<String> for Time {
    fn from(value: String) -> Self {
        let value = value.as_bytes();

        let hours = (value[1] - b'0') as i32 * 60 + (value[0] - b'0') as i32 * 10 * 60;
        let minutes = (value[4] - b'0' + (value[3] - b'0') * 10) as i32;

        Self(hours + minutes)
    }
}

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut times = time_points
            .into_iter()
            .map(|point| point.into())
            .collect::<Vec<Time>>();

        times.sort_unstable();

        let mut min_diff = i32::MAX;
        for chunk in times.windows(2) {
            min_diff = min_diff.min(chunk[1].0 - chunk[0].0);
        }

        if times.len() >= 2 {
            let max_time = 24 * 60;

            min_diff = min_diff.min(max_time - times[times.len() - 1].0 + times[0].0);
        }

        min_diff
    }
}

fn main() {
    let test_cases = [vec!["01:01", "02:01"]];

    for time_points in test_cases {
        println!(
            "{}",
            Solution::find_min_difference(time_points.into_iter().map(str::to_owned).collect())
        );
    }
}
