// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/description/
pub struct Solution;

impl Solution {
    pub fn get_lucky(mut s: String, k: i32) -> i32 {
        fn convert(s: &str) -> String {
            s.as_bytes()
                .iter()
                .map(|c| c - b'a' + 1)
                .map(|num| format!("{num}"))
                .fold(String::new(), |acc, s| format!("{acc}{s}"))
        }
        fn transform(s: &str) -> String {
            let num = s
                .as_bytes()
                .iter()
                .map(|digit| (digit - b'0') as i32)
                .sum::<i32>();
            format!("{num}")
        }

        s = convert(&s);
        for _ in 0..k {
            s = transform(&s);
        }

        s.parse::<i32>().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
