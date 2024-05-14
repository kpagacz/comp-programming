// https://leetcode.com/problems/permutation-difference-between-two-strings/description/
pub struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        use std::collections::HashMap;

        let t = t
            .as_bytes()
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (pos, &b)| {
                map.insert(b, pos);
                map
            });

        s.as_bytes()
            .iter()
            .enumerate()
            .map(|(pos, b)| (pos as i32 - *t.get(b).unwrap() as i32).abs())
            .sum::<i32>()
    }
}

fn main() {
    println!("Hello, world!");
}
