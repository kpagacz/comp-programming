// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/description/?envType=daily-question&envId=2026-03-01
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.as_bytes().iter().max().map(|c| c - b'0').unwrap() as _
    }
}

fn main() {
    println!("Hello, world!");
}
