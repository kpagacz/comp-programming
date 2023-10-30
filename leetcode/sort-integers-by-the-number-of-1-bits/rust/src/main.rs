// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/description/
pub struct Solution {}

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(|&first, &second| {
            Solution::count_bits(first)
                .cmp(&Solution::count_bits(second))
                .then(first.cmp(&second))
        });
        arr
    }

    fn count_bits(num: i32) -> i32 {
        let mut answer = 0;
        (0..32).for_each(|pos| {
            if (1 << pos) & num > 0 {
                answer += 1
            }
        });
        answer
    }
}
fn main() {
    println!("Hello, world!");
}
