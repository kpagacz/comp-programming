// https://leetcode.com/problems/largest-combination-with-bitwise-and-greater-than-zero/description/
pub struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut counts = vec![0; 32];

        for num in candidates {
            (0..32).for_each(|shift| {
                if (1 << shift) & num > 0 {
                    counts[shift] += 1;
                }
            });
        }

        counts.into_iter().max().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
