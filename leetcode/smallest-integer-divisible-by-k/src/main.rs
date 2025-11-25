// https://leetcode.com/problems/smallest-integer-divisible-by-k/description/?envType=daily-question&envId=2025-11-25
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        use std::collections::HashSet;
        let mut remainders = HashSet::new();
        let k = k as i64;
        let mut n = 1i64;
        let mut length = 1;

        loop {
            let new_remainder = n % k;
            if remainders.contains(&new_remainder) {
                return -1;
            } else if new_remainder == 0 {
                return length;
            }
            remainders.insert(new_remainder);
            n = ((n * 10) + 1) % k;
            length += 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
