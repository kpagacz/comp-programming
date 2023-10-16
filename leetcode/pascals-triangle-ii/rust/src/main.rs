// https://leetcode.com/problems/pascals-triangle-ii/description/
pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let rows = row_index as usize + 1;
        let mut row = vec![0; rows];
        row[0] = 1;
        (0..row_index).for_each(|_| {
            (1..row.len()).rev().for_each(|id| {
                row[id] = row[id] + row[id - 1];
            })
        });
        row
    }
}

fn main() {
    println!("Hello, world!");
}
