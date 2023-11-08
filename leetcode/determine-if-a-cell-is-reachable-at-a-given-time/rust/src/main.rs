// https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/description/
pub struct Solution {}

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let diagonal_moves = (fx - sx).abs().min((fy - sy).abs());
        let rest = (fx - sx).abs() - diagonal_moves + (fy - sy).abs() - diagonal_moves;

        if diagonal_moves + rest == 0 {
            t > 1 || t == 0
        } else {
            t >= diagonal_moves + rest
        }
    }
}

fn main() {
    println!("Hello, world!");
}
