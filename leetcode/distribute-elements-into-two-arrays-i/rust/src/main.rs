// https://leetcode.com/problems/distribute-elements-into-two-arrays-i/description/
pub struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = vec![];
        let mut arr2 = vec![];

        arr1.push(nums[0]);
        arr2.push(nums[1]);

        nums.iter().skip(2).for_each(|&num| {
            if arr1.last().unwrap() > arr2.last().unwrap() {
                arr1.push(num);
            } else {
                arr2.push(num);
            }
        });

        arr1.extend_from_slice(&arr2);
        arr1
    }
}

fn main() {
    println!("Hello, world!");
}
