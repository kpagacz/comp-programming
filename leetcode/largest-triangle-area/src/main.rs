// https://leetcode.com/problems/largest-triangle-area/description/?envType=daily-question&envId=2025-09-27
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut largest_area = f64::MIN;
        for first in 0..points.len() {
            for second in first + 1..points.len() {
                for third in second + 1..points.len() {
                    let (x1, y1) = (points[first][0] as f64, points[first][1] as f64);
                    let (x2, y2) = (points[second][0] as f64, points[second][1] as f64);
                    let (x3, y3) = (points[third][0] as f64, points[third][1] as f64);

                    let area =
                        (x1 * y2) + (x2 * y3) + (x3 * y1) - (y1 * x2) - (y2 * x3) - (y3 * x1);
                    let area = area.abs();
                    largest_area = largest_area.max(area);
                }
            }
        }
        largest_area / 2.0
    }
}

fn main() {
    println!("Hello, world!");
}
