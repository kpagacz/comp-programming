// https://leetcode.com/problems/range-addition-ii/description/
pub struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let dims = ops.iter().fold((m, n), |(min_row, min_col), op| {
            (min_row.min(op[0]), min_col.min(op[1]))
        });
        dims.0 * dims.1
    }
}

fn main() {
    println!("Hello, world!");
}
