// https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/description/
pub struct Solution;

impl Solution {
    pub fn max_count(mut banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut answer = 0;
        banned.sort_unstable();
        let mut current_sum = 0;
        for i in 1..=n {
            if banned.binary_search(&i).is_err() {
                if current_sum + i <= max_sum {
                    answer += 1;
                    current_sum += i;
                } else {
                    break;
                }
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
