// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-ii/description/
pub struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut flips = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        // Make palindromes
        // Take note that whether flipped to 0 or 1
        // it doesn't change the sum of all ones modulo 4
        // in a completely palindromic matrix
        for row in 0..grid.len() / 2 {
            for col in 0..grid[0].len() / 2 {
                let (first, second, third, fourth) = (
                    grid[row][col],
                    grid[rows - row - 1][col],
                    grid[rows - row - 1][cols - col - 1],
                    grid[row][cols - col - 1],
                );

                let sum = first + second + third + fourth;
                flips += i32::min((0 - sum).abs(), (4 - sum).abs());
            }
        }

        // Sum all the ones in the middle of odd dimensions
        let middle_row = rows / 2;
        let middle_col = cols / 2;

        let mut row_ones: i32 = 0;
        let mut middle_row_flips: i32 = 0;
        if rows % 2 == 1 {
            row_ones = grid[middle_row].iter().sum::<i32>();
            if cols % 2 == 1 {
                row_ones -= grid[middle_row][middle_col];
            }
            for col in 0..middle_col {
                if grid[middle_row][col] != grid[middle_row][cols - col - 1] {
                    middle_row_flips += 1;
                }
            }

            flips += middle_row_flips;
            if row_ones % 4 == 2 && middle_row_flips == 0 {
                flips += 2;
            }
        }

        let mut col_ones = 0;
        let mut middle_col_flips = 0;
        if cols % 2 == 1 {
            col_ones = grid.iter().map(|row| row[middle_col]).sum::<i32>();
            if rows % 2 == 1 {
                col_ones -= grid[middle_row][middle_col];
            }
            for row in 0..middle_row {
                if grid[row][middle_col] != grid[rows - row - 1][middle_col] {
                    middle_col_flips += 1;
                }
            }

            flips += middle_col_flips;
            if col_ones % 4 == 2 && middle_col_flips == 0 {
                flips += 2;
            }
        }

        if rows % 2 == 1 && cols % 2 == 1 {
            flips += grid[middle_row][middle_col];

            match (
                col_ones % 4 == 2 && middle_col_flips == 0,
                row_ones % 4 == 2 && middle_row_flips == 0,
            ) {
                (true, true) => flips -= 4,
                (true, false) => {
                    if middle_row_flips > 0 {
                        flips -= 2;
                    }
                }
                (false, true) => {
                    if middle_col_flips > 0 {
                        flips -= 2;
                    }
                }
                _ => {}
            }
        }

        flips
    }
}

fn main() {
    let test_cases = [
        vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]],
        vec![vec![0, 1], vec![0, 1], vec![0, 0]],
        vec![vec![1], vec![1]],
        vec![vec![1], vec![1], vec![1]],
        vec![
            vec![0, 0, 1],
            vec![0, 0, 1],
            vec![1, 0, 1],
            vec![1, 0, 0],
            vec![0, 1, 1],
        ],
        vec![vec![0, 1, 1], vec![0, 0, 0], vec![0, 1, 1]],
    ];

    for grid in test_cases {
        println!("{}", Solution::min_flips(grid));
    }
}
