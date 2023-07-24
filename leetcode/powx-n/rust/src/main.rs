// https://leetcode.com/problems/powx-n/

pub struct Solution {}

impl Solution {
    // Runtime 2ms
    // Beats 68.25%of users with Rust
    // Memory 2.02mb
    // Beats 60.32%of users with Rust
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut larger_n: i64 = n as i64;
        if larger_n == 0 {
            return 1.0;
        }
        if larger_n < 0 {
            x = 1.0 / x;
            larger_n *= -1;
        }

        let mut rest = 1.0;
        while larger_n > 1 {
            if larger_n % 2 == 1 {
                rest *= x;
            }
            x *= x;
            larger_n /= 2;
        }
        x * rest
    }
}

fn main() {
    let tests: Vec<(f64, i32)> = vec![
        (0.0, 0),
        (-3_f64, 3),
        (-3.0, 4),
        (1.0, 0),
        (1.0, 1000),
        (2.0, -4),
    ];
    for (x, n) in tests {
        println!("{}", Solution::my_pow(x, n));
    }
}
