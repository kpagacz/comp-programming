// https://leetcode.com/problems/sort-array-by-parity/
pub struct Solution {}
impl Solution {
    // That's the best solution. It's basically a partition algorithm.
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut pp = 0_usize;
        for it in 0..nums.len() {
            if nums[it] % 2 == 0 {
                nums.swap(pp, it);
                pp += 1;
            }
        }
        nums
    }
}
fn main() {
    println!("Hello, world!");
}
