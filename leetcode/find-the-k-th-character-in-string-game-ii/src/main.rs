// https://leetcode.com/problems/find-the-k-th-character-in-string-game-ii/description/?envType=daily-question&envId=2025-07-04
pub struct Solution;

impl Solution {
    pub fn kth_character(mut k: i64, operations: Vec<i32>) -> char {
        k -= 1;
        let mut shift = 0;
        let mut operations_it = 0;
        while k > 0 {
            if k & 1 == 1 && operations[operations_it] == 1 {
                shift += 1;
            }

            operations_it += 1;
            k >>= 1;
        }

        (b'a' + shift % 26) as char
    }
}

fn main() {
    println!("Hello, world!");
}
