// https://leetcode.com/problems/reshape-the-matrix/description/
pub struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;
        let (old_r, old_c) = (mat.len(), mat[0].len());

        if old_r * old_c != r * c {
            return mat;
        }

        let mut new = vec![vec![0; c]; r];

        for id in 0..r * c {
            let (old_row, old_col) = (id / old_c, id % old_c);
            let (new_row, new_col) = (id / c, id % c);
            new[new_row][new_col] = mat[old_row][old_col];
        }

        new
    }
}

fn main() {
    println!("Hello, world!");
}
