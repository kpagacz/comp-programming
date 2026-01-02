// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/description/?envType=daily-question&envId=2026-01-02
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counts = HashMap::new();
        let n = nums.len();
        for num in nums {
            counts
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for (num, count) in counts {
            if count == n / 2 {
                return num;
            }
        }
        unreachable!()
    }
}
fn main() {
    println!("Hello, world!");
}
