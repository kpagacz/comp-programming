// https://leetcode.com/problems/minimum-cost-to-convert-string-i/description/
pub struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        // 1. Change original, changed, cost to a 26x26 matrix with cost
        // 2. Run F-W on this matrix
        // 3. For each letter of source-target find the cost using 2. and if cost == -1, return -1
        // 4. Return the sum of the costs
        let mut costs = [[i64::MAX; 26]; 26];
        for i in 0..original.len() {
            let (from, to, cost) = (
                original[i] as u8 - b'a',
                changed[i] as u8 - b'a',
                cost[i] as i64,
            );
            let (from, to) = (from as usize, to as usize);
            costs[from][to] = costs[from][to].min(cost);
        }
        // println!("{costs:#?}");

        let costs = Self::floydwarshall(costs);
        // println!("{costs:#?}");

        let mut total_cost = 0;

        let source = source.as_bytes();
        let target = target.as_bytes();
        for i in 0..source.len() {
            let (from, to) = (source[i] - b'a', target[i] - b'a');
            let (from, to) = (from as usize, to as usize);
            if from == to {
                continue;
            }
            if costs[from][to] == i64::MAX {
                // println!("i: {i} can't map: {from} to {to}");
                return -1;
            } else {
                total_cost += costs[from][to];
            }
        }

        total_cost
    }

    fn floydwarshall(mut graph: [[i64; 26]; 26]) -> [[i64; 26]; 26] {
        for k in 0..graph.len() {
            for i in 0..graph.len() {
                for j in 0..graph.len() {
                    if graph[i][k] != i64::MAX && graph[k][j] != i64::MAX {
                        graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
                    }
                }
            }
        }
        graph
    }
}

fn main() {
    println!("Hello, world!");
}
