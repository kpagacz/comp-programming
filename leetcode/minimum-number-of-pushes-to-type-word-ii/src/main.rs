// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/description/
pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let w = word.as_bytes();
        let mut freqs = [0; 256];
        for &c in w {
            freqs[c as usize] += 1;
        }
        let mut sorted: Vec<usize> = (0..256).collect();
        use std::cmp::Reverse;
        sorted.sort_unstable_by_key(|&pos| Reverse(freqs[pos]));

        let mut answer = 0;
        const BUTTONS: usize = 8;

        println!("{freqs:?}");
        println!("{sorted:?}");
        for (idx, &c) in sorted.iter().enumerate() {
            let times_pressed = freqs[c];
            answer += times_pressed * (1 + (idx / BUTTONS))
        }

        answer as _
    }
}

fn main() {
    let test_cases = ["abcde"];

    for word in test_cases {
        println!("{}", Solution::minimum_pushes(word.to_string()));
    }
}
