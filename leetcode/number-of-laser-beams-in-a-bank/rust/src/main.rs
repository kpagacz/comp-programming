// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/
pub struct Solution;

impl Solution {
    pub fn number_of_beams(mut bank: Vec<String>) -> i32 {
        let mut answer = 0;
        let mut devices_to_pair = 0;
        while let Some(row) = bank.pop() {
            let devices = row.chars().filter(|&c| c == '1').count();
            if devices == 0 {
                continue;
            }
            answer += devices_to_pair * devices;
            devices_to_pair = devices;
        }
        answer as i32
    }
}

fn main() {
    println!("Hello, world!");
}
