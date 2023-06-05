use std::cmp::Ordering;

// https://leetcode.com/problems/check-if-it-is-a-straight-line/
pub struct Solution {}

impl Solution {
    pub fn check_straight_line(mut coordinates: Vec<Vec<i32>>) -> bool {
        let (x1, x2, y1, y2) = (
            coordinates[0][0],
            coordinates[1][0],
            coordinates[0][1],
            coordinates[1][1],
        );
        if x2 != x1 {
            let a = (y2 - y1) as f64 / (x2 - x1) as f64;
            let b = y1 as f64 - a * x1 as f64;

            coordinates.iter().fold(true, |is_line, point| {
                is_line && (a * point[0] as f64 + b - (point[1] as f64)).abs() < 1e-7
            })
        } else {
            coordinates
                .iter()
                .fold(true, |is_line, point| is_line && point[0] == x1)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
