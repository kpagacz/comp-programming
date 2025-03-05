// https://leetcode.com/problems/count-total-number-of-colored-cells/description/
pub struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        n * n + (n - 1) * (n - 1)
    }
}

fn main() {
    println!("Hello, world!");
}
