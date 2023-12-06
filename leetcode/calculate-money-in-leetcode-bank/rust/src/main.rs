// https://leetcode.com/problems/calculate-money-in-leetcode-bank/
pub struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut weeks = 0;
        let mut week_amount = 0;
        let mut total = 0;
        for i in 0..n {
            if i % 7 == 0 {
                weeks += 1;
                total += weeks;
                week_amount = weeks;
            } else {
                week_amount += 1;
                total += week_amount;
            }
        }
        total
    }
}
fn main() {
    println!("Hello, world!");
}
