// https://leetcode.com/problems/finding-3-digit-even-numbers/description/?envType=daily-question&envId=2025-05-12
pub struct Solution;

impl Solution {
    pub fn find_even_numbers(_digits: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![];
        let mut digits = [0; 10];
        for digit in _digits {
            digits[digit as usize] += 1;
        }

        for mut num in (100..=999).step_by(2) {
            let original = num;
            let mut num_digits = [0; 10];
            while num > 0 {
                num_digits[(num % 10) as usize] += 1;
                num /= 10;
            }
            let mut possible = true;
            for digit in 0..=9usize {
                possible = possible && (num_digits[digit] <= digits[digit]);
            }
            if possible {
                answer.push(original);
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
