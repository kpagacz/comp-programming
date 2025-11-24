// https://leetcode.com/problems/binary-prefix-divisible-by-5/description/?envType=daily-question&envId=2025-11-24
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut prefix = 0;
        nums.into_iter()
            .map(|num| {
                prefix <<= 1;
                prefix += num;
                prefix %= 5;
                prefix == 0
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
