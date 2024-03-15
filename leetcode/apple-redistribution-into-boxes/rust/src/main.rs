// https://leetcode.com/problems/apple-redistribution-into-boxes/description/
pub struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_unstable_by(|a, b| b.cmp(a));

        let mut apples = apple.iter().sum::<i32>();

        let mut it = 0;
        while apples > 0 {
            apples -= capacity[it];
            it += 1;
        }

        it as _
    }
}

fn main() {
    println!("Hello, world!");
}
