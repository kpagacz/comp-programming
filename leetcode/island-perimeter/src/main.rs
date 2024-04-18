// https://leetcode.com/problems/island-perimeter/description/
pub struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    perimeter += 4;
                    if i > 0 && grid[i - 1][j] == 1 {
                        perimeter -= 2;
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        perimeter -= 2;
                    }
                }
            }
        }

        perimeter
    }
}

fn main() {
    println!("Hello, world!");
}
