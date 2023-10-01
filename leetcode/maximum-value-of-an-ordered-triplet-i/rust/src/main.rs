// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-i/
pub struct Solution {}
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max = i64::MIN;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                for k in (j + 1)..nums.len() {
                    max = max.max((nums[i] - nums[j]) as i64 * nums[k] as i64);
                }
            }
        }
        max.max(0)
    }
}
fn main() {
    println!("Hello, world!");
}
