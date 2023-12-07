// https://leetcode.com/problems/largest-odd-number-in-string/
pub struct Solution {}

const ODD_DIGITS: [u8; 5] = [b'1', b'3', b'5', b'7', b'9'];
impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let bytes = num.as_bytes();
        let pos = bytes.len()
            - bytes
                .iter()
                .rev()
                .position(|digit| ODD_DIGITS.contains(digit))
                .unwrap_or(bytes.len());
        num[..pos].to_owned()
    }
}

fn main() {
    println!("Hello, world!");
}
