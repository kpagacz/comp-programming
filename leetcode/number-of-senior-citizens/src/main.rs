// https://leetcode.com/problems/number-of-senior-citizens/description/
pub struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .filter(|detail| detail[11..13].parse::<i32>().unwrap() > 60)
            .count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
