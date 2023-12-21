// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/description/
pub struct Solution {}

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[0]);
        points
            .windows(2)
            .map(|window| window[1][0] - window[0][0])
            .max()
            .unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
