// https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/description/?envType=daily-question&envId=2026-03-18
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_submatrices(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                let prev = if row == 0 { 0 } else { grid[row - 1][col] };
                grid[row][col] += prev;
            }
        }

        let mut answer = 0;

        for row in 0..grid.len() {
            let mut sum = 0;
            for column_prefix in grid[row].iter().take(grid[0].len()) {
                sum += column_prefix;
                if sum <= k {
                    answer += 1;
                }
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
