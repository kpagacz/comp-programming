// https://leetcode.com/problems/maximum-total-importance-of-roads/description/
pub struct Solution;

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut degrees = vec![0; n as usize];
        for road in &roads {
            let (from, to) = (road[0] as usize, road[1] as usize);
            degrees[from] += 1;
            degrees[to] += 1;
        }

        degrees.sort_unstable();

        let mut answer = 0;

        for i in 1..n as usize {
            answer += degrees[i - 1] * i;
        }

        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}
