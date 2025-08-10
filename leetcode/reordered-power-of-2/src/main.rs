// https://leetcode.com/problems/reordered-power-of-2/submissions/?envType=daily-question&envId=2025-08-10
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut powers_of_two_digits = vec![];

        fn num_to_digits(mut num: i32) -> Vec<usize> {
            let mut digits = vec![0; 10];
            while num > 0 {
                digits[(num % 10) as usize] += 1;
                num /= 10;
            }
            digits
        }

        let mut num = 1;
        for _ in 0..32 {
            powers_of_two_digits.push(num_to_digits(num));
            num <<= 1;
        }

        powers_of_two_digits.contains(&num_to_digits(n))
    }
}

fn main() {
    println!("Hello, world!");
}
