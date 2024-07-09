// https://leetcode.com/problems/average-waiting-time/description/
pub struct Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut total_waiting_time = 0f64;
        let mut last_arrival = customers[0][0] as f64;
        let mut current_wait_time = 0f64;
        let n = customers.len() as f64;

        for customer in customers {
            let (arrival, prep_time) = (customer[0] as f64, customer[1] as f64);
            current_wait_time = f64::max(current_wait_time - (arrival - last_arrival), 0f64);
            current_wait_time += prep_time;
            total_waiting_time += current_wait_time;
            last_arrival = arrival;
        }

        total_waiting_time / n
    }
}

fn main() {
    let test_cases = [vec![vec![1, 2], vec![2, 5], vec![4, 3]]];
    for customers in test_cases {
        println!("{}", Solution::average_waiting_time(customers));
    }
}
