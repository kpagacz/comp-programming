// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/
pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s = s.as_bytes();
        let mut suffix_as = s.iter().filter(|c| **c == b'a').count() as i32;
        let mut prefix_bs = 0;

        let mut min_ops = suffix_as;
        for &c in s {
            if c == b'a' {
                suffix_as -= 1;
            } else {
                prefix_bs += 1;
            }

            min_ops = min_ops.min(suffix_as + prefix_bs);
        }
        min_ops
    }
}

fn main() {
    println!("Hello, world!");
}
