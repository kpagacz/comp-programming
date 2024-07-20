// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/description/
pub struct Solution;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut curr_row_sum = vec![0; row_sum.len()];
        let mut curr_col_sum = vec![0; col_sum.len()];

        let mut answer = vec![vec![0; col_sum.len()]; row_sum.len()];

        for row in 0..answer.len() {
            for col in 0..answer[0].len() {
                answer[row][col] = i32::min(
                    row_sum[row] - curr_row_sum[row],
                    col_sum[col] - curr_col_sum[col],
                );
                curr_row_sum[row] += answer[row][col];
                curr_col_sum[col] += answer[row][col];
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [(vec![3, 8], vec![4, 7]), (vec![1, 0], vec![1])];
    for (row_sum, col_sum) in test_cases {
        println!("case: {row_sum:?} {col_sum:?}");
        println!("{:#?}", Solution::restore_matrix(row_sum, col_sum));
    }
}
