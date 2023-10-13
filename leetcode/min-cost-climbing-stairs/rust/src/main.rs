// https://leetcode.com/problems/min-cost-climbing-stairs/
pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let mut min_cost = vec![i32::MAX; cost.len() + 2];
        min_cost[0] = 0;
        min_cost[1] = 0;

        (0..cost.len()).for_each(|step| {
            let new_cost = min_cost[step] + cost[step];
            min_cost[step + 1] = min_cost[step + 1].min(new_cost);
            min_cost[step + 2] = min_cost[step + 2].min(new_cost);
        });

        min_cost[cost.len()]
    }
}
fn main() {
    println!("Hello, world!");
}
