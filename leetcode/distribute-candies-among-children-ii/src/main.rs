// https://leetcode.com/problems/distribute-candies-among-children-ii/description/?envType=daily-question&envId=2025-06-01
pub struct Solution;

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        Self::cal(n + 2) - 3 * Self::cal(n - limit + 1) + 3 * Self::cal(n - (limit + 1) * 2 + 2)
            - Self::cal(n - 3 * (limit + 1) + 2)
    }

    fn cal(x: i32) -> i64 {
        if x < 0 {
            0
        } else {
            x as i64 * (x - 1) as i64 / 2
        }
    }
}

fn main() {
    println!("Hello, world!");
}
