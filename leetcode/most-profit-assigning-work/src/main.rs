// https://leetcode.com/problems/most-profit-assigning-work/description/
pub struct Solution;

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        use std::collections::BinaryHeap;
        let mut jobs = difficulty
            .into_iter()
            .zip(profit)
            .map(|(diff, profit)| (profit, diff))
            .collect::<BinaryHeap<_>>();
        worker.sort_unstable_by_key(|num| -num);

        let mut answer = 0;
        for worker_skill in worker {
            while let Some(&(profit, diff)) = jobs.peek() {
                if worker_skill >= diff {
                    answer += profit;
                    break;
                } else {
                    jobs.pop();
                }
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
