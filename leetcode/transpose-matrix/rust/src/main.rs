// https://leetcode.com/problems/transpose-matrix/description/
pub struct Solution {}

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = vec![vec![0; matrix.len()]; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                answer[j][i] = matrix[i][j];
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
