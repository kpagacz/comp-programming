// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-i/description/?envType=daily-question&envId=2025-08-22
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut min_row, mut min_col, mut max_row, mut max_col) =
            (usize::MAX, usize::MAX, usize::MIN, usize::MIN);

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    min_row = min_row.min(row);
                    min_col = min_col.min(col);
                    max_row = max_row.max(row);
                    max_col = max_col.max(col);
                }
            }
        }

        ((max_row - min_row + 1) * (max_col - min_col + 1)) as _
    }
}

fn main() {
    println!("Hello, world!");
}
