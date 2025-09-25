// https://leetcode.com/problems/triangle/description/?envType=daily-question&envId=2025-09-25
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for row in 1..triangle.len() {
            for col in 0..=row {
                triangle[row][col] += if col == 0 {
                    triangle[row - 1][col]
                } else if col == row {
                    triangle[row - 1][col - 1]
                } else {
                    triangle[row - 1][col].min(triangle[row - 1][col - 1])
                };
            }
        }
        *triangle.last().unwrap().into_iter().min().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
