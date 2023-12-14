// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/description/
pub struct Solution {}

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_ones: Vec<i32> = grid
            .iter()
            .map(|row| row.iter().filter(|&&num| num == 1).count() as i32)
            .collect();
        let grid_rows = grid.len() as i32;
        let grid_cols = grid[0].len() as i32;

        fn transpose(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut answer = vec![vec![0; grid.len()]; grid[0].len()];
            for i in 0..answer.len() {
                for j in 0..answer[0].len() {
                    answer[i][j] = grid[j][i];
                }
            }
            answer
        }

        let transposed = transpose(grid);
        let col_ones: Vec<i32> = transposed
            .iter()
            .map(|row| row.iter().filter(|&&num| num == 1).count() as i32)
            .collect();

        let mut answer = vec![vec![0; grid_cols as usize]; grid_rows as usize];

        for i in 0..answer.len() {
            for j in 0..answer[0].len() {
                answer[i][j] = 2 * row_ones[i] + 2 * col_ones[j] - grid_rows - grid_cols;
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
