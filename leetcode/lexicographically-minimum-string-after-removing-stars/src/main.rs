// https://leetcode.com/problems/lexicographically-minimum-string-after-removing-stars/description/?envType=daily-question&envId=2025-06-07
pub struct Solution;

impl Solution {
    pub fn clear_stars(s: String) -> String {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut max_heap = BinaryHeap::new();

        for (pos, c) in s.chars().enumerate() {
            if c == '*' {
                max_heap.pop();
            } else {
                max_heap.push((Reverse(c), pos));
            }
        }

        let mut underlying = max_heap.into_vec();
        underlying.sort_by_key(|(_, pos)| *pos);
        underlying
            .into_iter()
            .map(|(Reverse(c), _)| c)
            .collect::<String>()
    }
}

fn main() {
    println!("Hello, world!");
}
