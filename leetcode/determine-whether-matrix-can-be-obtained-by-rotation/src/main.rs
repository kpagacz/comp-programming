// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/description/?envType=daily-question&envId=2026-03-22
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();

        for _ in 0..4 {
            for row in 0..n {
                for col in 0..n {
                    let temp = mat[row][col];
                    mat[row][col] = mat[n - 1 - col][row];
                    mat[n - 1 - col][row] = mat[n - 1 - row][n - 1 - col];
                    mat[n - 1 - row][n - 1 - col] = mat[col][n - 1 - row];
                    mat[col][n - 1 - row] = temp;
                }
            }

            if mat == target {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
