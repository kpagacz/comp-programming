// https://leetcode.com/problems/sum-of-square-numbers/description/
pub struct Solution;

impl Solution {
    // https://math.libretexts.org/Bookshelves/Combinatorics_and_Discrete_Mathematics/Elementary_Number_Theory_(Barrus_and_Clark)/01%3A_Chapters/1.28%3A_Sum_of_Squares
    pub fn judge_square_sum(mut c: i32) -> bool {
        use std::collections::HashMap;
        let mut factorization = HashMap::new();

        let mut it = 2i32;
        let limit = c;
        while it.saturating_mul(it) <= limit {
            if c % it == 0 {
                *factorization.entry(it).or_insert(0) += 1;
                c /= it;
            } else {
                it += 1;
            }
        }
        if c != 1 {
            *factorization.entry(c).or_insert(0) += 1;
        }

        for (divisor, count) in factorization {
            if divisor % 4 == 3 && count % 2 == 1 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let test_cases = [5, 3, 98, 99, 100, 2147483646];
    for c in test_cases {
        println!("{}", Solution::judge_square_sum(c));
    }
}
