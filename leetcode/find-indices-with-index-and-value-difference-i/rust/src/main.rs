// find-indices-with-index-and-value-difference-i
pub struct Solution {}

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let index_difference = index_difference as usize;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if j - i >= index_difference && (nums[j] - nums[i]).abs() >= value_difference {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![-1, -1]
    }
}
fn main() {
    println!("Hello, world!");
}
