// https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/description/?envType=daily-question&envId=2025-06-15
pub struct Solution;

impl Solution {
    pub fn max_diff(mut num: i32) -> i32 {
        fn num_from_digits(digits: &[i32]) -> i32 {
            digits
                .iter()
                .fold((0, 1), |(answer, mul), &digit| {
                    (answer + digit * mul, mul * 10)
                })
                .0
        }
        fn replace_in_vector(vec: &mut [i32], to_replace: i32, replacement: i32) {
            vec.iter_mut().for_each(|digit| {
                if *digit == to_replace {
                    *digit = replacement;
                }
            });
        }

        let mut digits = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        let mut max = digits.clone();
        let to_replace = *max.iter().rev().find(|&&digit| digit != 9).unwrap_or(&9);
        replace_in_vector(&mut max, to_replace, 9);
        let max_after_replacement = num_from_digits(&max);

        let (to_replace, replacement) = if *digits.last().unwrap() == 1 {
            let to_replace = *digits
                .iter()
                .rev()
                .find(|&&digit| digit != 0 && digit != 1)
                .unwrap_or(&1);

            (to_replace, if to_replace == 1 { 1 } else { 0 })
        } else {
            (*digits.last().unwrap(), 1)
        };
        replace_in_vector(&mut digits, to_replace, replacement);
        let min_after_replacement = num_from_digits(&digits);

        max_after_replacement - min_after_replacement
    }
}

fn main() {
    let test_cases = [
        (555, 888),
        (9, 8),
        (1019, 8089),
        (9019, 8908),
        (9154220, 8800000),
    ];
    for (num, exp) in test_cases {
        println!("TEST: {num}");
        println!("{} exp {exp}", Solution::max_diff(num));
    }
}
