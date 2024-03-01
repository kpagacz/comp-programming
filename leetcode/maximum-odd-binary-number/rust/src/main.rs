// https://leetcode.com/problems/maximum-odd-binary-number/description/
pub struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let s = s.as_bytes();
        let len = s.len();
        let ones = s.iter().filter(|&&b| b == b'1').count();

        std::iter::repeat('1')
            .take(ones - 1)
            .chain(std::iter::repeat('0').take(len - ones))
            .chain(std::iter::once('1'))
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
