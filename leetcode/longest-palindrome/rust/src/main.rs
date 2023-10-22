// https://leetcode.com/problems/longest-palindrome/
pub struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let freq = s.as_bytes().iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });
        freq.values().fold(0, |acc, count| {
            acc + if count % 2 == 1 { count - 1 } else { *count }
        }) + if freq.values().any(|count| *count % 2 == 1) {
            1
        } else {
            0
        }
    }
}
fn main() {
    println!("Hello, world!");
}
