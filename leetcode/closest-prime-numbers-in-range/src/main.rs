// https://leetcode.com/problems/closest-prime-numbers-in-range/description/
pub struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut is_prime = vec![true; right as usize + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        let sqr = f32::sqrt(left as f32).ceil() as i32;
        // println!("sqrt: {sqr}");
        for i in 2..=sqr as usize {
            if is_prime[i] {
                for not_prime in (2 * i..=right as usize).step_by(i) {
                    is_prime[not_prime] = false;
                }
            }
        }

        let (mut first_prime, mut second_prime, mut min_delta, mut last_prime) =
            (-1, -1, i32::MAX, i32::MIN);
        // println!("{is_prime:?}");
        for (num, prime) in is_prime.into_iter().enumerate().skip(left as usize) {
            let num = num as i32;
            if prime {
                if last_prime == i32::MIN {
                    last_prime = num;
                    continue;
                }

                let delta = num - last_prime;
                if delta < min_delta {
                    first_prime = last_prime;
                    second_prime = num;
                    min_delta = delta;
                }
                last_prime = num;
            }
        }
        vec![first_prime, second_prime]
    }
}

fn main() {
    let test_cases = [(10, 19)];
    for (left, right) in test_cases {
        println!("{:?}", Solution::closest_primes(left, right));
    }
}
