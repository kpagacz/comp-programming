// https://leetcode.com/problems/largest-submatrix-with-rearrangements/description/?envType=daily-question&envId=2026-03-17
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if matrix[row][col] == 0 {
                    matrix[row][col] = 0;
                } else {
                    let prev = if row == 0 { 0 } else { matrix[row - 1][col] };
                    matrix[row][col] = prev + 1;
                }
            }
        }

        matrix
            .into_iter()
            .map(|mut row| {
                row.sort_by(|a, b| b.cmp(a));
                row.into_iter()
                    .enumerate()
                    .map(|(pos, val)| (pos as i32 + 1) * val)
                    .max()
                    .expect("Matrix is non empty!")
            })
            .max()
            .expect("Matrix is non empty!")
    }
}

fn main() {}
