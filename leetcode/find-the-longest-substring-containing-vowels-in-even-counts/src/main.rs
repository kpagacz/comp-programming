// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/description/
pub struct Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut count = (0, 0, 0, 0, 0); // a, e, i ,o ,u
        use std::collections::HashMap;
        let mut first_occurance = HashMap::new();
        first_occurance.insert((true, true, true, true, true), 0);

        let s = s.as_bytes();
        let mut answer = 0;
        for (pos, c) in s.iter().enumerate() {
            match *c {
                b'a' => count.0 += 1,
                b'e' => count.1 += 1,
                b'i' => count.2 += 1,
                b'o' => count.3 += 1,
                b'u' => count.4 += 1,
                _ => {}
            }
            let seen_characters = pos + 1;

            let key = (
                count.0 % 2 == 0,
                count.1 % 2 == 0,
                count.2 % 2 == 0,
                count.3 % 2 == 0,
                count.4 % 2 == 0,
            );
            if let Some(&already_seen) = first_occurance.get(&key) {
                answer = answer.max(seen_characters - already_seen);
            } else {
                first_occurance.insert(key, seen_characters);
            }
        }

        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}
