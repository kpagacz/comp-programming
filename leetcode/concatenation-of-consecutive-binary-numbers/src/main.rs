// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/description/?envType=daily-question&envId=2026-02-28
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;
        let mut pow = 1i64;
        let mut answer = 0i64;
        for mut num in (1..=n).rev() {
            while num > 0 {
                if num & 1 == 1 {
                    answer += pow;
                }
                pow <<= 1;
                pow %= MOD;
                num >>= 1;
            }
        }
        (answer % MOD) as _
    }
}

fn main() {
    println!("Hello, world!");
}
