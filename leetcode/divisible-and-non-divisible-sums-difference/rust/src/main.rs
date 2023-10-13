// https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/
pub struct Solution {}

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let (mut num1, mut num2) = (0, 0);
        (1..=n).for_each(|num| {
            if num % m == 0 {
                num2 += num;
            } else {
                num1 += num;
            }
        });
        num1 - num2
    }
}

fn main() {
    println!("Hello, world!");
}
