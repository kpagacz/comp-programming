// https://leetcode.com/problems/largest-magic-square/description/?envType=daily-question&envId=2026-01-18
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let mut col_prefixes = vec![vec![0; grid.len() + 1]; grid[0].len()];
        let mut row_prefixes = vec![vec![0; grid[0].len() + 1]; grid.len()];
        let mut forward_diagonal_prefixes = grid.clone();
        let mut backward_diagonal_prefixes = grid.clone();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                row_prefixes[i][j + 1] = row_prefixes[i][j] + grid[i][j];
                col_prefixes[j][i + 1] = col_prefixes[j][i] + grid[i][j];
                if i > 0 && j > 0 {
                    forward_diagonal_prefixes[i][j] += forward_diagonal_prefixes[i - 1][j - 1];
                }
                if i > 0 && j < grid[0].len() - 1 {
                    backward_diagonal_prefixes[i][j] += backward_diagonal_prefixes[i - 1][j + 1];
                }
            }
        }

        fn are_all_elements_equal(mut it: impl Iterator<Item = i32>) -> bool {
            let first = it.next();
            it.fold(first, |acc, num| {
                acc.and_then(|stored| if stored == num { Some(stored) } else { None })
            })
            .is_some()
        }

        let row_sums = |start_row: usize, end_row: usize, col_start: usize, col_end: usize| {
            row_prefixes[start_row..=end_row]
                .iter()
                .map(move |row| row[col_end + 1] - row[col_start])
        };

        let col_sums = |start_row: usize, end_row: usize, col_start: usize, col_end: usize| {
            col_prefixes[col_start..=col_end]
                .iter()
                .map(move |col| col[end_row + 1] - col[start_row])
        };

        let diagonal_sums = |i: usize, j: usize, bottom_right_i: usize, bottom_right_j: usize| {
            let before_forward_diag = if i > 0 && j > 0 {
                forward_diagonal_prefixes[i - 1][j - 1]
            } else {
                0
            };
            let before_backward_diag = if i > 0 && bottom_right_j < grid[0].len() - 1 {
                backward_diagonal_prefixes[i - 1][bottom_right_j + 1]
            } else {
                0
            };

            [
                forward_diagonal_prefixes[bottom_right_i][bottom_right_j] - before_forward_diag,
                backward_diagonal_prefixes[bottom_right_i][j] - before_backward_diag,
            ]
            .into_iter()
        };

        let is_square_magic = |i: usize, j: usize, end: (usize, usize)| {
            are_all_elements_equal(
                row_sums(i, end.0, j, end.1)
                    .chain(col_sums(i, end.0, j, end.1))
                    .chain(diagonal_sums(i, j, end.0, end.1)),
            )
        };

        let mut max_size = 1;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut end = (i + 1, j + 1);
                while end.0 < grid.len() && end.1 < grid[0].len() {
                    if is_square_magic(i, j, end) {
                        max_size = max_size.max(end.0 - i + 1);
                    }
                    end = (end.0 + 1, end.1 + 1);
                }
            }
        }

        max_size as _
    }
}

fn main() {
    let test_cases = vec![(
        vec![
            vec![7, 1, 4, 5, 6],
            vec![2, 5, 1, 6, 4],
            vec![1, 5, 4, 3, 2],
            vec![1, 2, 7, 3, 4],
        ],
        3,
    )];
    for (grid, exp) in test_cases {
        println!("{} exp {exp}", Solution::largest_magic_square(grid));
    }
}
