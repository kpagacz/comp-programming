// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/description/
pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots_weak(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut answer = 1;
        let mut shooting_interval = points[0].clone();
        for point in &points[1..] {
            if Solution::do_intervals_cross(&shooting_interval, point) {
                shooting_interval[0] = shooting_interval[0].max(point[0]);
                shooting_interval[1] = shooting_interval[1].min(point[1]);
            } else {
                answer += 1;
                shooting_interval = point.clone();
            }
        }

        answer
    }

    fn do_intervals_cross(a: &[i32], b: &[i32]) -> bool {
        a[0] <= b[1] && b[0] <= a[1]
    }

    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut answer = 1;
        let mut last_end = points[0][1];

        for point in &points[1..] {
            // start of the last interval <= end of the current interval is guaranteed
            // by the sorting and the fact that the intervals are well formed (StartX <= EndX)
            // which means we only need to check whether the start of the current interval
            // is <= than the last interval
            if point[0] > last_end {
                answer += 1;
                last_end = point[1];
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
