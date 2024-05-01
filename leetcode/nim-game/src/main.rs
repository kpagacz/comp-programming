// https://leetcode.com/problems/nim-game/description/
struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

fn main() {
    let test_cases = [4, 1, 2];
    for n in test_cases {
        println!("{}", Solution::can_win_nim(n));
    }
}
