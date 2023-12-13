// https://leetcode.com/problems/special-positions-in-a-binary-matrix/
pub struct Solution {}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1
                    && mat[i][..j].iter().all(|&el| el == 0)
                    && mat[i][j + 1..].iter().all(|&el| el == 0)
                    && mat[..i].iter().all(|el| el[j] == 0)
                    && mat[i + 1..].iter().all(|el| el[j] == 0)
                {
                    answer += 1;
                }
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
