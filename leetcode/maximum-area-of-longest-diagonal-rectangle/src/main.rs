// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/description/?envType=daily-question&envId=2025-08-26

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions
            .into_iter()
            .map(|rectangle| {
                (
                    rectangle[0] * rectangle[0] + rectangle[1] * rectangle[1],
                    rectangle[0] * rectangle[1],
                )
            })
            .max()
            .unwrap()
            .1
    }
}

fn main() {
    println!("Hello, world!");
}
