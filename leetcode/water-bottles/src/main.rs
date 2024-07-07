// https://leetcode.com/problems/water-bottles/description/
pub struct Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut answer = 0;
        let mut empty_bottles = 0;

        while num_bottles > 0 {
            answer += num_bottles;
            num_bottles += empty_bottles;
            empty_bottles = num_bottles % num_exchange;
            num_bottles /= num_exchange;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
