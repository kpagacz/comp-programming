// https://leetcode.com/problems/move-zeroes/
pub struct Solution {}
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut no_zeros_up_to, mut it) = (-1, 0);
        while it < nums.len() {
            if nums[it] != 0 {
                no_zeros_up_to += 1;
                nums.swap(it, no_zeros_up_to as usize);
            }
            it += 1;
        }
    }
}
fn main() {
    println!("Hello, world!");
}
