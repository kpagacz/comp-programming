// https://leetcode.com/problems/robot-return-to-origin/description/
pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut position = (0, 0);

        moves.chars().for_each(|c| match c {
            'R' => position.1 += 1,
            'L' => position.1 -= 1,
            'U' => position.0 -= 1,
            'D' => position.0 += 1,
            _ => unreachable!(),
        });

        position.0 == 0 && position.1 == 0
    }
}

fn main() {
    println!("Hello, world!");
}
