// https://leetcode.com/problems/lucky-numbers-in-a-matrix/description/
pub struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let min_in_row: Vec<i32> = (0..matrix.len())
            .map(|row| *matrix[row].iter().min().unwrap())
            .collect();
        let max_in_column: Vec<i32> = (0..matrix[0].len())
            .map(|col| (0..matrix.len()).map(|row| matrix[row][col]).max().unwrap())
            .collect();

        let mut answer = vec![];
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if matrix[row][col] == min_in_row[row] && matrix[row][col] == max_in_column[col] {
                    answer.push(matrix[row][col]);
                }
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
