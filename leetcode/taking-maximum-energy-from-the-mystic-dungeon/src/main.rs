// https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon/description/
pub struct Solution;

impl Solution {
    pub fn maximum_energy(mut energy: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        for i in (0..energy.len()).rev() {
            if i + k < energy.len() {
                energy[i] += energy[i + k];
            }
        }

        *energy.iter().max().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
