// https://leetcode.com/problems/snake-in-matrix/description/
pub struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut position = 0;
        for command in commands {
            match &command[..] {
                "UP" => position -= n,
                "DOWN" => position += n,
                "LEFT" => position -= 1,
                _ => position += 1,
            }
        }
        position
    }
}

fn main() {
    println!("Hello, world!");
}
