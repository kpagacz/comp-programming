// https://leetcode.com/problems/divide-array-into-equal-pairs/description/?envType=daily-question&envId=2025-03-17
pub struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        if nums.len() & 1 == 1 {
            false
        } else {
            let mut arr: [i32; 501] = [0; 501];
            for num in nums {
                arr[num as usize] += 1;
            }

            for count in arr {
                if count & 1 == 1 {
                    return false;
                }
            }
            true
        }
    }
}

fn main() {
    println!("Hello, world!");
}
