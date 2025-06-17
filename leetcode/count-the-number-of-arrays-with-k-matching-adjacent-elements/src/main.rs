// https://leetcode.com/problems/count-the-number-of-arrays-with-k-matching-adjacent-elements/description/?envType=daily-question&envId=2025-06-17
pub struct Solution;

impl Solution {
    // Computes (a ^ exp) % m
    fn pow(mut a: i64, mut exp: i64, m: i64) -> i64 {
        let mut res = 1;
        while exp > 0 {
            if exp & 1 == 1 {
                res = (res * a) % m;
            }
            a = (a * a) % m;
            exp >>= 1;
        }
        res
    }
    // Computes (n choose k) modulo m
    // Uses some nifty math properties:
    // (n choose k) = n! / (k! * (n - k)!)
    fn n_choose_k(n: i64, k: i64, m: i64) -> i64 {
        let mut factorials = vec![1; n as usize + 1];
        let mut inv_factorials = vec![1; n as usize + 1];

        for i in 1..factorials.len() {
            factorials[i] = (factorials[i - 1] * i as i64) % m;
        }

        inv_factorials[n as usize] = Self::pow(factorials[factorials.len() - 1], m - 2, m);
        for i in (1..inv_factorials.len()).rev() {
            inv_factorials[i - 1] = (inv_factorials[i] * i as i64) % m;
        }

        factorials[n as usize] * inv_factorials[k as usize] % m
            * inv_factorials[n as usize - k as usize]
            % m
    }
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;
        let n = n as i64;
        let m = m as i64;
        let k = k as i64;

        ((((m * Self::n_choose_k(n - 1, k, MOD)) % MOD) * Self::pow(m - 1, n - k - 1, MOD)) % MOD)
            as _
    }
}

fn main() {
    println!("Hello, world!");
}
