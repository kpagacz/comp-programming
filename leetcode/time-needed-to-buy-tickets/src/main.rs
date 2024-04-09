// https://leetcode.com/problems/time-needed-to-buy-tickets/description/
pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut answer = 0;
        for &person in &tickets[0..k] {
            answer += person.min(tickets[k]);
        }
        for &person in &tickets[k + 1..] {
            answer += person.min(tickets[k] - 1);
        }

        answer + tickets[k]
    }
}

fn main() {
    let test_cases = [
        (vec![2, 3, 2], 2),
        (vec![5, 1, 1, 1], 0),
        (vec![4, 2, 3, 1, 5], 2),
    ];
    for (tickets, k) in test_cases {
        println!("{}", Solution::time_required_to_buy(tickets, k));
    }
}
