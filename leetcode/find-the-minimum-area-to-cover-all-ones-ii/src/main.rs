// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii/description/?envType=daily-question&envId=2025-08-23
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut min_area = i32::MAX;

        // Vertical split
        for right_start in 1..grid[0].len() {
            // Additional vertical split
            let leftest_area = Self::minimum_area(&grid, 0, grid.len(), 0, right_start - 1);
            for more_right_start in right_start + 1..grid[0].len() {
                let middle_area =
                    Self::minimum_area(&grid, 0, grid.len(), right_start, more_right_start - 1);
                let right_area =
                    Self::minimum_area(&grid, 0, grid.len(), more_right_start, grid[0].len());
                min_area = min_area.min(leftest_area + middle_area + right_area);
            }

            // Additional horizontal split on the left
            let right_area = Self::minimum_area(&grid, 0, grid.len(), right_start, grid[0].len());
            for horizontal_split in 1..grid.len() {
                let left_top =
                    Self::minimum_area(&grid, 0, horizontal_split - 1, 0, right_start - 1);
                let left_bottom =
                    Self::minimum_area(&grid, horizontal_split, grid.len(), 0, right_start - 1);
                min_area = min_area.min(right_area + left_top + left_bottom);
            }

            // Additional horizontal split on the right
            let left_area = Self::minimum_area(&grid, 0, grid.len(), 0, right_start - 1);
            for horizontal_split in 1..grid.len() {
                let right_top =
                    Self::minimum_area(&grid, 0, horizontal_split - 1, right_start, grid[0].len());
                let right_bottom = Self::minimum_area(
                    &grid,
                    horizontal_split,
                    grid.len(),
                    right_start,
                    grid[0].len(),
                );
                min_area = min_area.min(left_area + right_top + right_bottom);
            }
        }

        let grid = Self::transpose(grid);
        for right_start in 1..grid[0].len() {
            // Additional vertical split
            let leftest_area = Self::minimum_area(&grid, 0, grid.len(), 0, right_start - 1);
            for more_right_start in right_start + 1..grid[0].len() {
                let middle_area =
                    Self::minimum_area(&grid, 0, grid.len(), right_start, more_right_start - 1);
                let right_area =
                    Self::minimum_area(&grid, 0, grid.len(), more_right_start, grid[0].len());
                min_area = min_area.min(leftest_area + middle_area + right_area);
            }

            // Additional horizontal split on the left
            let right_area = Self::minimum_area(&grid, 0, grid.len(), right_start, grid[0].len());
            for horizontal_split in 1..grid.len() {
                let left_top =
                    Self::minimum_area(&grid, 0, horizontal_split - 1, 0, right_start - 1);
                let left_bottom =
                    Self::minimum_area(&grid, horizontal_split, grid.len(), 0, right_start - 1);
                min_area = min_area.min(right_area + left_top + left_bottom);
            }

            // Additional horizontal split on the right
            let left_area = Self::minimum_area(&grid, 0, grid.len(), 0, right_start - 1);
            for horizontal_split in 1..grid.len() {
                let right_top =
                    Self::minimum_area(&grid, 0, horizontal_split - 1, right_start, grid[0].len());
                let right_bottom = Self::minimum_area(
                    &grid,
                    horizontal_split,
                    grid.len(),
                    right_start,
                    grid[0].len(),
                );
                min_area = min_area.min(left_area + right_top + right_bottom);
            }
        }

        min_area
    }

    fn transpose(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut transposed = vec![vec![0; grid.len()]; grid[0].len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                transposed[j][i] = grid[i][j];
            }
        }

        transposed
    }

    fn minimum_area(
        grid: &[Vec<i32>],
        min_row: usize,
        max_row: usize,
        min_col: usize,
        max_col: usize,
    ) -> i32 {
        let (mut minrow, mut mincol, mut maxrow, mut maxcol) =
            (usize::MAX, usize::MAX, usize::MIN, usize::MIN);

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if row >= min_row
                    && row <= max_row
                    && col >= min_col
                    && col <= max_col
                    && grid[row][col] == 1
                {
                    minrow = minrow.min(row);
                    mincol = mincol.min(col);
                    maxrow = maxrow.max(row);
                    maxcol = maxcol.max(col);
                }
            }
        }

        ((maxrow - minrow + 1) * (maxcol - mincol + 1)) as _
    }
}

fn main() {
    println!("Hello, world!");
}
