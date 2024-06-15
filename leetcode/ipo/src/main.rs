// https://leetcode.com/problems/ipo/description/
pub struct Solution;

impl Solution {
    pub fn find_maximized_capital(
        mut k: i32,
        mut w: i32,
        profits: Vec<i32>,
        capital: Vec<i32>,
    ) -> i32 {
        let mut projects_by_capital = (0..profits.len()).collect::<Vec<_>>();
        projects_by_capital.sort_unstable_by_key(|&project| capital[project]);
        use std::collections::BinaryHeap;

        let mut available_projects_by_profit = BinaryHeap::new();
        let mut not_available_start = 0;

        while k > 0 {
            while not_available_start < projects_by_capital.len()
                && capital[projects_by_capital[not_available_start]] <= w
            {
                let project = projects_by_capital[not_available_start];

                available_projects_by_profit.push(profits[project]);
                not_available_start += 1;
            }

            if let Some(net_profit) = available_projects_by_profit.pop() {
                w += net_profit;
                k -= 1;
            } else {
                break;
            }
        }

        w
    }
}

fn main() {
    println!("Hello, world!");
}
