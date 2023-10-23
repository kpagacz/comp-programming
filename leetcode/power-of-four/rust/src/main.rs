// https://leetcode.com/problems/power-of-four/description/
pub struct Solution {}
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // 0b01010101010101010101010101010101 == 0x55555555
        n > 0 && (n & (n - 1)) == 0 && n & 0x55555555 == n
    }
    pub fn is_power_of_four_dumb(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let bytes = format!("{n:b}");
        let zeros = bytes.chars().rev().take_while(|c| c == &'0').count();
        bytes.len() - zeros == 1 && zeros % 2 == 0
    }
}
fn main() {
    println!("{}", Solution::is_power_of_four(16));
    println!("{}", Solution::is_power_of_four(32));
    println!("{}", Solution::is_power_of_four(1));
}
