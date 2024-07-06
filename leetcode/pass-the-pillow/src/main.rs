// https://leetcode.com/problems/pass-the-pillow/description/
pub struct Solution;

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let m = 2 * n - 2;
        let rest = time % m;

        if rest < n {
            1 + rest
        } else {
            (n - 1) - (rest - n)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
