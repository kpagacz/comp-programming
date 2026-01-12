// https://leetcode.com/problems/minimum-time-visiting-all-points/
struct Solution {}
#[allow(dead_code)]
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|adjacent| {
                let (first, second) = (&adjacent[0], &adjacent[1]);
                let (x_diff, y_diff) = ((first[0] - second[0]).abs(), (first[1] - second[1]).abs());
                x_diff.min(y_diff) + (x_diff - y_diff).abs()
            })
            .sum()
    }
}

fn main() {
    println!("Hello, world!");
}
