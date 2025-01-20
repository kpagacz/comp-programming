// https://leetcode.com/problems/first-completely-painted-row-or-column/description/
pub struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut row_paints = vec![mat[0].len(); mat.len()];
        let mut col_paints = vec![mat.len(); mat[0].len()];

        use std::collections::HashMap;
        let mut num_to_pos = HashMap::new();
        mat.into_iter().enumerate().for_each(|(row_id, row)| {
            row.into_iter().enumerate().for_each(|(col_id, num)| {
                num_to_pos.insert(num, (row_id, col_id));
            })
        });

        for (id, painted) in arr.into_iter().enumerate() {
            let (row, col) = *num_to_pos.get(&painted).unwrap();
            row_paints[row] -= 1;
            col_paints[col] -= 1;
            if row_paints[row] == 0 || col_paints[col] == 0 {
                return id as _;
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
