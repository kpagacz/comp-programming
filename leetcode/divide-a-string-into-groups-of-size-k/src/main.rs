// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/description/?envType=daily-question&envId=2025-06-22
pub struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        s.as_bytes()
            .chunks(k as usize)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|b| *b as char)
                    .chain(std::iter::repeat(fill))
                    .take(k as usize)
                    .collect::<String>()
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
