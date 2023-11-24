//https://leetcode.com/contest/weekly-contest-372/problems/separate-black-and-white-balls/
pub struct Solution {}

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut hops = 0i64;
        let mut ones = 0i64;
        let s = s.as_bytes();
        for b in s {
            if b == &b'1' {
                ones += 1;
            } else {
                hops += ones;
            }
        }
        hops
    }
}

fn main() {
    println!("Hello, world!");
}
