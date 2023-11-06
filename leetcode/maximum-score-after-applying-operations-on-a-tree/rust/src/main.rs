pub struct Solution {}

impl Solution {
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        let destinations = edges
            .into_iter()
            .fold(vec![vec![]; values.len()], |mut dests, edge| {
                dests[edge[0] as usize].push(edge[1] as usize);
                dests[edge[1] as usize].push(edge[0] as usize);
                dests
            });

        fn recursive_min_cost(
            node: usize,
            parent: usize,
            destinations: &Vec<Vec<usize>>,
            values: &Vec<i32>,
        ) -> i64 {
            if destinations[node].len() == 0
                || (destinations[node].len() == 1 && destinations[node][0] == parent)
            {
                return values[node] as i64;
            }
            destinations[node]
                .iter()
                .filter(|&&dest| dest != parent)
                .map(|&destination| recursive_min_cost(destination, node, destinations, values))
                .sum::<i64>()
                .min(values[node] as i64)
        }

        values.iter().fold(0, |acc, &value| acc + value as i64)
            - recursive_min_cost(0, 0, &destinations, &values)
    }
}
fn main() {
    println!("Hello, world!");
}
