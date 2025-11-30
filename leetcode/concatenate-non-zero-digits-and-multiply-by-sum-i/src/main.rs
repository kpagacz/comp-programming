// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-i/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut sum = 0i64;
        let mut x = 0i64;
        let mut pow = 0;
        while n > 0 {
            let rest = n % 10;
            n /= 10;
            if rest != 0 {
                sum += rest as i64;
                x += rest as i64 * 10i64.pow(pow);
                pow += 1;
            }
        }
        x * sum
    }
}
fn main() {
    println!("Hello, world!");
}
