// https://leetcode.com/problems/prime-number-of-set-bits-in-binary-representation/description/?envType=daily-question&envId=2026-02-21
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        const SMALL_PRIMES: [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 31];
        (left..=right)
            .filter_map(|num| {
                if SMALL_PRIMES.contains(&num.count_ones()) {
                    Some(1)
                } else {
                    None
                }
            })
            .sum::<i32>()
    }
}

fn main() {
    println!("Hello, world!");
}
