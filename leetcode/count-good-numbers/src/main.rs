// https://leetcode.com/problems/count-good-numbers/description/?envType=daily-question&envId=2025-04-13
pub struct Solution;

impl Solution {
    pub fn count_good_numbers(n: i64) -> i64 {
        const MOD: i64 = 10i64.pow(9) + 7;

        fn fast_exp(mut base: i64, mut exp: i64) -> i64 {
            let mut answer = 1i64;

            while exp > 0 {
                if exp % 2 == 1 {
                    answer = answer * base % MOD;
                }
                base = (base * base) % MOD;
                exp /= 2;
            }

            answer
        }

        let even_choices = 5i64;
        let odd_choices = 4i64;
        let even_digits = (n + 1) / 2;
        let odd_digits = n / 2;

        let even_possibilities = fast_exp(even_choices, even_digits);
        let odd_possibilities = fast_exp(odd_choices, odd_digits);

        (even_possibilities * odd_possibilities) % MOD
    }
}

fn main() {
    println!("Hello, world!");
}
