// https://leetcode.com/problems/count-operations-to-obtain-zero/description/?envType=daily-question&envId=2025-11-09
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        if num1 == 0 || num2 == 0 {
            return 0;
        }

        if num1 >= num2 {
            1 + Solution::count_operations(num1 - num2, num2)
        } else {
            1 + Solution::count_operations(num1, num2 - num1)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
