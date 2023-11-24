// https://leetcode.com/problems/maximum-number-of-coins-you-can-get/description/
pub struct Solution {}

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let times = piles.len() / 3;
        let mut sum = 0;
        for i in 0..times {
            sum += piles[piles.len() - 2 - 2 * i];
        }
        sum
    }

    pub fn max_coins_smarter(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        let len = piles.len();
        piles.into_iter().skip(len / 3).step_by(2).sum()
    }
}

fn main() {
    println!("Hello, world!");
}
