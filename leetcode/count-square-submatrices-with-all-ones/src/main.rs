// https://leetcode.com/problems/count-square-submatrices-with-all-ones/description/
pub struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let cols = matrix[0].len();
        let rows = matrix.len();

        let row_prefixes = matrix
            .iter()
            .map(|row| {
                std::iter::once(0)
                    .chain(row.iter().scan(0, |acc, num| {
                        *acc += num;
                        Some(*acc)
                    }))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let col_prefixes = (0..cols)
            .map(|col| {
                std::iter::once(0)
                    .chain((0..rows).scan(0, |acc, row_id| {
                        *acc += matrix[row_id][col];
                        Some(*acc)
                    }))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut answer = 0;

        for x in 0..rows {
            for y in 0..cols {
                for side in 1.. {
                    if x + side <= rows && y + side <= cols {
                        if row_prefixes[x + side - 1][y + side] - row_prefixes[x + side - 1][y]
                            != side as i32
                        {
                            break;
                        }
                        if col_prefixes[y + side - 1][x + side] - col_prefixes[y + side - 1][x]
                            != side as i32
                        {
                            break;
                        }
                        answer += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        answer
    }

    pub fn count_squares_dp(mut matrix: Vec<Vec<i32>>) -> i32 {
        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 1 {
                    matrix[i][j] += matrix[i - 1][j]
                        .min(matrix[i][j - 1])
                        .min(matrix[i - 1][j - 1]);
                }
            }
        }

        matrix
            .into_iter()
            .fold(0, |acc, row| acc + row.into_iter().sum::<i32>())
    }
}

fn main() {
    println!("Hello, world!");
}
