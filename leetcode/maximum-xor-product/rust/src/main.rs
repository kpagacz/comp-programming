// https://leetcode.com/contest/weekly-contest-372/problems/maximum-xor-product/
pub struct Solution {}

const MOD: i64 = 10i64.pow(9) + 7;
impl Solution {
    pub fn maximum_xor_product(mut a: i64, mut b: i64, n: i32) -> i32 {
        for pos in (0..n).rev() {
            let prop = 1 << pos;
            if a.min(b) & prop == 0 {
                a ^= prop;
                b ^= prop;
            }
        }

        ((((a % MOD) * (b % MOD)) % MOD) % MOD) as i32
    }
}

fn main() {
    println!("Hello, world!");
}
