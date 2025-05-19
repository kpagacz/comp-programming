// https://leetcode.com/problems/type-of-triangle/description/?envType=daily-question&envId=2025-05-19
pub struct Solution;

impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort();
        if nums[0] + nums[1] <= nums[2] {
            return String::from("none");
        }
        if nums[0] == nums[2] {
            return String::from("equilateral");
        }
        if nums[1] == nums[2] || nums[0] == nums[1] {
            return String::from("isosceles");
        }
        String::from("scalene")
    }
}

fn main() {
    println!("Hello, world!");
}
