// https://leetcode.com/problems/minimum-length-of-anagram-concatenation/description/
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let s = s.as_bytes();
        let freq = s.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });

        let mut curr = HashMap::new();
        for (pos, &b) in s.iter().enumerate() {
            *curr.entry(b).or_insert(0) += 1;
            if Solution::can_concatenate(&curr, &freq) {
                return pos as i32 + 1;
            }
        }

        unreachable!()
    }

    fn can_concatenate(curr: &HashMap<u8, i32>, target: &HashMap<u8, i32>) -> bool {
        let mut divs = target.iter().map(|(key, value)| match curr.get(key) {
            None => -1,
            Some(in_curr) => {
                if value % in_curr != 0 {
                    -1
                } else {
                    value / in_curr
                }
            }
        });
        let first = divs.clone().next().unwrap_or(-1);
        divs.all(|div| div == first && div != -1)
    }
}

fn main() {
    println!("Hello, world!");
}
