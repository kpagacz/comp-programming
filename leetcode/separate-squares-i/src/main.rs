// https://leetcode.com/problems/separate-squares-i/description/?envType=daily-question&envId=2026-01-13
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let (mut left, mut right) = (0f64, 10f64.powi(10));
        const EPSILON: f64 = 0.00001;

        while left + EPSILON < right {
            let mid = left + (right - left) / 2.0;

            let (below_area, above_area) =
                squares
                    .iter()
                    .fold((0f64, 0f64), |(below_area, above_area), square| {
                        let (y, l) = (square[1] as f64, square[2] as f64);
                        if y > mid {
                            // Square above the line
                            (below_area, above_area + l * l)
                        } else {
                            // Square under or crossing line
                            if y + l > mid {
                                // crossing
                                (below_area + l * (mid - y), above_area + l * (y + l - mid))
                            } else {
                                // wholly under
                                (below_area + l * l, above_area)
                            }
                        }
                    });

            if below_area < above_area {
                left = mid + EPSILON;
            } else {
                right = mid;
            }
        }

        left
    }
}

fn main() {
    println!("Hello, world!");
}
