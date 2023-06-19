// https://leetcode.com/problems/find-the-highest-altitude/

pub struct Solution {}

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        gain.iter()
            .scan(0, |cum, change| {
                *cum += change;
                Some(*cum)
            })
            .chain(vec![0])
            .max()
            .unwrap()
    }
}
fn main() {
    println!("Hello, world!");
}
