// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/description/?envType=daily-question&envId=2025-03-08
pub struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut whites = 0;
        let blocks = blocks.as_bytes();

        for block in &blocks[..k as usize] {
            if block == &b'W' {
                whites += 1;
            }
        }
        let mut min_recolors = whites;

        for i in k as usize..blocks.len() {
            if blocks[i - k as usize] == b'W' {
                whites -= 1;
            }
            if blocks[i] == b'W' {
                whites += 1;
            }
            min_recolors = min_recolors.min(whites);
        }
        min_recolors
    }
}

fn main() {
    println!("Hello, world!");
}
