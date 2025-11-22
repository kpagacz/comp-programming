// https://leetcode.com/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/submissions/1836962723/?envType=daily-question&envId=2025-11-22
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|num| match num % 3 {
                0 => 0,
                1 | 2 => 1,
                _ => unreachable!(),
            })
            .sum()
    }
}

fn main() {
    println!("Hello, world!");
}
