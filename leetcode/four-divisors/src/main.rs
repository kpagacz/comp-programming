// https://leetcode.com/problems/four-divisors/description/?envType=daily-question&envId=2026-01-04
struct Solution;

use std::collections::BTreeSet;
#[allow(dead_code)]
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let primes = Self::sieve();

        let mut sum = 0;
        for num in nums {
            for &prime in &primes {
                if prime > (num as f32).sqrt().floor() as i32 {
                    break;
                }
                if num % prime == 0
                    && num / prime != prime
                    && (primes.contains(&(num / prime)) || (num / prime) == (prime * prime))
                {
                    sum += 1 + num + prime + num / prime;
                    break;
                }
                if num % prime == 0 {
                    break;
                }
            }
        }
        sum
    }

    fn sieve() -> BTreeSet<i32> {
        let mut is_prime = vec![true; 100000];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..is_prime.len() {
            if is_prime[i] {
                for j in (2 * i..is_prime.len()).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        is_prime
            .into_iter()
            .enumerate()
            .filter_map(|(pos, is_prime)| if is_prime { Some(pos as i32) } else { None })
            .collect::<BTreeSet<i32>>()
    }
}

fn main() {
    let test_cases = [
        (vec![21, 4, 7], 32),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 45),
        (vec![7286, 18704, 70773, 8224, 91675], 10932),
    ];
    for (nums, exp) in test_cases {
        println!("{} exp {exp}", Solution::sum_four_divisors(nums));
    }
}
