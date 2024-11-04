// https://leetcode.com/problems/string-compression-iii/description/
pub struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut comp = String::default();
        let it = word.chars();
        let mut last_char = word.chars().next().unwrap();
        let mut compressed = 0;

        for c in it {
            if compressed == 9 {
                comp += &format!("{compressed}{last_char}");
                compressed = 0;
                last_char = c;
            }

            if last_char == c {
                compressed += 1;
            } else {
                comp += &format!("{compressed}{last_char}");
                last_char = c;
                compressed = 1;
            }
        }
        comp += &format!("{compressed}{last_char}");

        comp
    }
}

fn main() {
    println!("Hello, world!");
}
