// https://leetcode.com/problems/height-checker/description/
pub struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort_unstable();

        expected
            .into_iter()
            .zip(heights)
            .filter(|item| item.0 != item.1)
            .count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
