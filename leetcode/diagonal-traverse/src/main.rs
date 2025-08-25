// https://leetcode.com/problems/diagonal-traverse/description/?envType=daily-question&envId=2025-08-25

#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = vec![];
        let mut increment_rows = false;
        for distance in 0..=(mat.len() + mat[0].len() - 2) {
            if increment_rows {
                for row in 0..mat.len() {
                    let col = distance - row;
                    if col < mat[0].len() {
                        answer.push(mat[row][col]);
                    }
                }
            } else {
                for row in (0..mat.len()).rev() {
                    let col = distance - row;
                    if col < mat[0].len() {
                        answer.push(mat[row][col]);
                    }
                }
            }
            increment_rows = !increment_rows;
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
