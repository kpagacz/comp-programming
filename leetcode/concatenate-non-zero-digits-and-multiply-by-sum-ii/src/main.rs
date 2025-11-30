// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-ii/description/
struct Solution;

// There's a very clever trick here to remember, which is:
// it's possible to construct the number in O(1) from a prefix
// array (if modulo arithmetic is involved).
#[allow(dead_code)]
impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 10i64.pow(9) + 7;
        let s = s.as_bytes();
        let mut prefix_number = vec![0i64; s.len() + 1];
        let mut prefix_digit_sum = vec![0i64; s.len() + 1];
        let mut non_zero_counts = vec![0; s.len() + 1];
        let mut modulo_pow_10 = vec![1i64; s.len() + 1];
        for i in 1..modulo_pow_10.len() {
            modulo_pow_10[i] = (modulo_pow_10[i - 1] * 10) % MOD;
        }

        for (pos, &digit) in s.iter().enumerate() {
            let digit = (digit - b'0') as i64;

            prefix_number[pos + 1] = prefix_number[pos];
            prefix_digit_sum[pos + 1] = prefix_digit_sum[pos];
            non_zero_counts[pos + 1] = non_zero_counts[pos];

            if digit != 0 {
                prefix_number[pos + 1] = ((prefix_number[pos] * 10) + digit) % MOD;
                prefix_digit_sum[pos + 1] = prefix_digit_sum[pos] + digit;
                non_zero_counts[pos + 1] = non_zero_counts[pos] + 1;
            }
        }

        queries
            .into_iter()
            .map(|query| {
                let (l, r) = (query[0] as usize, query[1] as usize);
                let sum = prefix_digit_sum[r + 1] - prefix_digit_sum[l];
                let non_zeros = non_zero_counts[r + 1] - non_zero_counts[l];
                if non_zeros == 0 {
                    0
                } else {
                    let left_tail = (prefix_number[l] * modulo_pow_10[non_zeros]) % MOD;
                    let x = (prefix_number[r + 1] - left_tail + MOD) % MOD;
                    ((x * sum) % MOD) as _
                }
            })
            .collect()
    }
}
fn main() {
    println!("Hello, world!");
}
