// https://leetcode.com/problems/keep-multiplying-found-values-by-two/description/?envType=daily-question&envId=2025-11-19
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        use std::collections::HashSet;
        let nums: HashSet<i32> = HashSet::from_iter(nums);
        while nums.contains(&original) {
            original <<= 1;
        }
        original
    }
}
fn main() {
    println!("Hello, world!");
}
