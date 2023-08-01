// https://leetcode.com/problems/combinations/description/
pub struct Solution {}

impl Solution {
    // Runtime 3ms Beats 98.31% of users with Rust
    // Memory 2.90mb Beats 38.98% of users with Rust
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combinations = vec![];
        let mut base_array = vec![];
        Self::combinations_rec_backtracking(&mut base_array, 1, k, &mut combinations, n);
        combinations
    }

    fn combinations_rec_backtracking(
        arr: &mut Vec<i32>,
        it: i32,
        to_take: i32,
        combinations: &mut Vec<Vec<i32>>,
        max: i32,
    ) {
        if to_take == 0 {
            combinations.push(arr.to_vec());
            return;
        }
        if to_take > max - it + 1 {
            return;
        }
        for i in it..=max {
            arr.push(i);
            Self::combinations_rec_backtracking(arr, i + 1, to_take - 1, combinations, max);
            arr.pop();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
