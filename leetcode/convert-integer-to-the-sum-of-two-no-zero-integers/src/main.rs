// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/description/?envType=daily-question&envId=2025-09-08
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for first in 1..n {
            let other = n - first;
            if Solution::is_non_zero(first) && Solution::is_non_zero(other) {
                return vec![first, other];
            }
        }
        unreachable!("A solution is guaranteed");
    }

    fn is_non_zero(mut i: i32) -> bool {
        while i > 0 {
            if i % 10 == 0 {
                return false;
            }
            i /= 10;
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
