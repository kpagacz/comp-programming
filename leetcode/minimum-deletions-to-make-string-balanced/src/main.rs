// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/
pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let s = s.as_bytes();
        let prefix_bs = s
            .iter()
            .fold((vec![], 0), |(mut acc, bs), c| {
                acc.push(bs);
                if *c == b'b' {
                    (acc, bs + 1)
                } else {
                    (acc, bs)
                }
            })
            .0;
        let suffix_as = s
            .iter()
            .enumerate()
            .rev()
            .fold((vec![0; s.len()], 0), |(mut acc, aas), (pos, c)| {
                acc[pos] = aas;
                if *c == b'a' {
                    (acc, aas + 1)
                } else {
                    (acc, aas)
                }
            })
            .0;

        let mut answer = i32::MAX;

        for i in 0..s.len() {
            answer = i32::min(answer, prefix_bs[i] + suffix_as[i]);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
