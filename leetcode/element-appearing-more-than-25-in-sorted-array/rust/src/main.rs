// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array/
pub struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut current_int = -1;
        let mut current_count = 0;
        let limit = arr.len() / 4;
        for i in arr {
            if i == current_int {
                current_count += 1;
            } else {
                current_int = i;
                current_count = 1;
            }

            if current_count > limit {
                return current_int;
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
