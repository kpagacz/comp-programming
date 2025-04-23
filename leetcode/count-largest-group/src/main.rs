// https://leetcode.com/problems/count-largest-group/description/?envType=daily-question&envId=2025-04-23
pub struct Solution;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        use std::collections::HashMap;

        fn sum_digits(mut num: i32) -> i32 {
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            sum
        }

        let mut freqs = HashMap::new();
        for num in 1..=n {
            freqs
                .entry(sum_digits(num))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let max_freq = freqs.values().copied().max().unwrap_or(0);
        freqs.values().filter(|&&v| v == max_freq).count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
