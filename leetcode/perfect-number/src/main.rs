// https://leetcode.com/problems/perfect-number/description/
pub struct Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == -1 {
            return false;
        }
        let mut sum = 0;

        let sqrt = (num as f32).sqrt() as i32;
        for i in 2..=sqrt {
            if num % i == 0 {
                sum += i + num / i;
            }
        }

        sum + 1 == num
    }
}

fn main() {
    println!("Hello, world!");
}
