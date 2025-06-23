// https://leetcode.com/problems/sum-of-k-mirror-numbers/description/?envType=daily-question&envId=2025-06-23
pub struct Solution;

// What did I learn here?
// This is an extremely smart way of creating palindromes
// Sidesteps this whole thing with odd numbers by just
// dividing the base 10 when generating odd palindromes.
// Abusing the fact that overlapping the last digit
// is actually creating an odd palindrome.
struct KPalindromeIter {
    base: i64,
    min_base: i64,
    max_base: i64,
    generating_odd: bool,
    k: i64,
}
impl KPalindromeIter {
    fn new(k: i64) -> Self {
        Self {
            base: 1,
            min_base: 1,
            max_base: 9,
            generating_odd: true,
            k,
        }
    }

    fn is_k_palindrome(&self, mut num: i64) -> bool {
        let mut digits = vec![];
        while num > 0 {
            digits.push(num % self.k);
            num /= self.k;
        }
        for i in 0..(digits.len() / 2) {
            if digits[i] != digits[digits.len() - 1 - i] {
                return false;
            }
        }
        true
    }

    fn make_palindrome(&self) -> i64 {
        let mut combined = self.base;
        let mut x = self.base;
        if self.generating_odd {
            x /= 10;
        }
        while x > 0 {
            combined *= 10;
            combined += x % 10;
            x /= 10;
        }
        combined
    }

    fn increment_base(&mut self) {
        self.base += 1;
        if self.base > self.max_base {
            if self.generating_odd {
                self.generating_odd = false;
                self.base = self.min_base;
            } else {
                self.generating_odd = true;
                self.min_base *= 10;
                self.max_base = self.max_base * 10 + 9;
                self.base = self.min_base;
            }
        }
    }
}
impl Iterator for KPalindromeIter {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let base_ten_palindrome = self.make_palindrome();
            if self.is_k_palindrome(base_ten_palindrome) {
                self.increment_base();
                return Some(base_ten_palindrome);
            } else {
                self.increment_base();
            }
        }
    }
}

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let palindromic_iter = KPalindromeIter::new(k as _);
        palindromic_iter.take(n as usize).sum::<i64>()
    }
}

fn main() {
    let test_cases = [(2, 5, 25), (9, 30, 18627530), (3, 7, 499)];
    for (k, n, exp) in test_cases {
        println!("{} | {exp}", Solution::k_mirror(k, n));
    }
}
