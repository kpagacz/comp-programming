// https://leetcode.com/problems/fraction-addition-and-subtraction/description/
pub struct Solution;

use std::fmt::Display;
#[derive(Clone, Copy, Debug, PartialEq)]
enum Sign {
    Positive,
    Negative,
}
impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sign::Positive => "",
                Sign::Negative => "-",
            }
        )
    }
}

#[derive(Debug)]
struct Fraction(Sign, i64, i64); // Sign, Nominator, Denominator
impl Fraction {
    fn add(f1: Fraction, f2: Fraction) -> Fraction {
        let Fraction(sign1, mut nominator1, denominator1) = f1;
        let Fraction(sign2, mut nominator2, denominator2) = f2;

        let lcm = Self::lcm(denominator1, denominator2);
        nominator1 =
            nominator1 * (lcm / denominator1) * if sign1 == Sign::Negative { -1 } else { 1 };
        nominator2 =
            nominator2 * (lcm / denominator2) * if sign2 == Sign::Negative { -1 } else { 1 };

        let mut sum = nominator1 + nominator2;
        let final_sign = if sum < 0 {
            sum *= -1;
            Sign::Negative
        } else {
            Sign::Positive
        };
        let gcd = Self::gcd(sum, lcm);
        Fraction(final_sign, sum / gcd, lcm / gcd)
    }

    fn gcd(mut first: i64, mut second: i64) -> i64 {
        while second != 0 {
            let old_first = first;
            first = second;
            second = old_first % second;
        }
        first
    }

    fn lcm(first: i64, second: i64) -> i64 {
        first * second / Self::gcd(first, second)
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Sign::Positive => write!(f, "{}/{}", self.1, self.2),
            Sign::Negative => write!(f, "-{}/{}", self.1, self.2),
        }
    }
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut fractions = vec![];
        let mut it = 0;
        let expression = expression.as_bytes();

        let mut current_sign = Sign::Positive;
        while it < expression.len() {
            match expression[it] {
                b'0'..=b'9' => {
                    let mut num_start = it;
                    while expression[it] != b'/' {
                        it += 1;
                    }
                    let mut past_num = it;
                    let nominator = std::str::from_utf8(&expression[num_start..past_num])
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();
                    it += 1;
                    num_start = it;
                    while it < expression.len() && expression[it].is_ascii_digit() {
                        it += 1;
                    }
                    past_num = it;
                    let denominator = std::str::from_utf8(&expression[num_start..past_num])
                        .unwrap()
                        .parse::<i64>()
                        .unwrap();

                    fractions.push(Fraction(current_sign, nominator, denominator));
                }
                b'+' => {
                    current_sign = Sign::Positive;
                    it += 1
                }
                b'-' => {
                    current_sign = Sign::Negative;
                    it += 1
                }
                _ => unreachable!(),
            }
        }
        format!("{}", fractions.into_iter().reduce(Fraction::add).unwrap())
    }
}

fn main() {
    let test_cases = ["-1/2+1/2", "-1/2+1/2+1/3", "1/3-1/2"];

    for expression in test_cases {
        println!("{}", Solution::fraction_addition(expression.to_string()));
    }
}
