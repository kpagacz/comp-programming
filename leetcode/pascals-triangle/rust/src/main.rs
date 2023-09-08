// https://leetcode.com/problems/pascals-triangle/description/
pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut pascal = vec![vec![]; num_rows as usize];
        pascal[0].push(1);
        for i in 1..(num_rows as usize) {
            pascal[i].push(1);
            for j in 1..=(i - 1) {
                let new = pascal[i - 1][j - 1] + pascal[i - 1][j];
                pascal[i].push(new);
            }
            pascal[i].push(1);
        }

        pascal
    }
}

fn main() {
    println!("Hello, world!");
}
