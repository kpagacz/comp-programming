// https://leetcode.com/problems/count-of-matches-in-tournament/description/
pub struct Solution {}

impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut matches = 0;
        while n > 1 {
            if n % 2 == 0 {
                matches += n / 2;
                n /= 2;
            } else {
                matches += (n - 1) / 2;
                n = (n - 1) / 2 + 1;
            }
        }
        matches
    }
}

fn main() {
    println!("Hello, world!");
}
