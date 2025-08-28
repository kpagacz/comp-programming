// https://leetcode.com/problems/sort-matrix-by-diagonals
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn sort_diagonal(row: usize, col: usize, increasing: bool, grid: &mut [Vec<i32>]) {
            let mut diagonal = vec![];
            let (mut row_it, mut col_it) = (row, col);

            while row_it < grid.len() && col_it < grid[0].len() {
                diagonal.push(grid[row_it][col_it]);
                row_it += 1;
                col_it += 1;
            }

            if increasing {
                diagonal.sort_unstable();
            } else {
                diagonal.sort_by_key(|num| std::cmp::Reverse(*num));
            }

            row_it = row;
            col_it = col;
            for num in diagonal.into_iter().rev() {
                grid[row_it][col_it] = num;
                row_it += 1;
                col_it += 1;
            }
        }

        let rows = grid.len();
        let cols = grid[0].len();
        for row in 0..rows {
            sort_diagonal(row, 0, true, &mut grid);
        }

        for col in 1..cols {
            sort_diagonal(0, col, false, &mut grid);
        }

        grid
    }
}

fn main() {
    println!("Hello, world!");
}
