// https://leetcode.com/problems/minimum-penalty-for-a-shop/description/?envType=daily-question&envId=2025-12-26
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut min_penalty = customers.as_bytes().iter().filter(|&&c| c == b'Y').count() as i32;
        let mut min_penalty_hour = 0;
        let mut ys = min_penalty;
        let mut ns = 0;
        for (hours, customer) in customers.as_bytes().iter().enumerate() {
            if *customer == b'Y' {
                ys -= 1;
            } else {
                ns += 1;
            }

            if ns + ys < min_penalty {
                min_penalty = ns + ys;
                min_penalty_hour = hours + 1;
            }
        }
        min_penalty_hour as _
    }
}

fn main() {
    println!("Hello, world!");
}
