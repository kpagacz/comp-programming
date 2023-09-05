// https://leetcode.com/problems/count-symmetric-integers/
pub struct Solution {}

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high)
            .filter(|&el| Self::is_symmetric_integer(el))
            .count() as i32
    }

    fn is_symmetric_integer(mut i: i32) -> bool {
        let mut digits = vec![];
        while i > 0 {
            digits.push(i % 10);
            i = i / 10;
        }
        if digits.len() % 2 == 1 {
            return false;
        }

        digits.iter().take(digits.len() / 2).sum::<i32>()
            == digits.iter().skip(digits.len() / 2).sum::<i32>()
    }
}

fn main() {
    println!("Hello, world!");
}
