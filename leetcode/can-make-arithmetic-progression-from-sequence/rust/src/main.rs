// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/description/

pub struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        arr.windows(3)
            .fold(true, |is_arithmetic_progression, window| {
                is_arithmetic_progression && (window[1] - window[0] == window[2] - window[1])
            })
    }
}
fn main() {
    println!("Hello, world!");
}
