// https://leetcode.com/problems/find-if-array-can-be-sorted/description/
pub struct Solution;

impl Solution {
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        for i in (0..nums.len()).rev() {
            let mut j = i;
            while j < nums.len() - 1 && nums[j] > nums[j + 1] {
                if nums[j].count_ones() == nums[j + 1].count_ones() {
                    nums.swap(j, j + 1);
                } else {
                    return false;
                }
                j += 1;
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
