// https://leetcode.com/problems/diagonal-traverse-ii/description/
pub struct Solution {}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr: Vec<(usize, usize, i32)> = nums
            .iter()
            .enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, &num)| (x, y, num)))
            .collect();
        arr.sort_unstable_by(|first, second| {
            (first.0 + first.1)
                .cmp(&(second.0 + second.1))
                .then(second.0.cmp(&first.0))
        });
        arr.into_iter().map(|(_, _, num)| num).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
