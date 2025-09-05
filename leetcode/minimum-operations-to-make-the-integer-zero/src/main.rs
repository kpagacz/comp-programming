// https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/description/?envType=daily-question&envId=2025-09-05
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let mut k = 1i64;

        loop {
            let x = num1 as i64 - (k * num2 as i64);

            if x < k {
                return -1;
            } else if k >= x.count_ones().into() {
                return k as _;
            }
            k += 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
