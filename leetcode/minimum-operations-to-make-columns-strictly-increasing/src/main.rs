// https://leetcode.com/problems/minimum-operations-to-make-columns-strictly-increasing/description/
pub struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut ops = 0;
        for col in 0..grid[0].len() {
            let mut previous = grid[0][col] - 1;
            for row in 0..grid.len() {
                if grid[row][col] <= previous {
                    ops += previous + 1 - grid[row][col];
                    previous += 1;
                } else {
                    previous = grid[row][col];
                }
            }
        }
        ops
    }
}

fn main() {
    println!("Hello, world!");
}
