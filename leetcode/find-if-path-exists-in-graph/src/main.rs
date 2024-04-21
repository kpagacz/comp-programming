// https://leetcode.com/problems/find-if-path-exists-in-graph/description/
pub struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        use std::collections::HashMap;
        let edges: HashMap<i32, Vec<i32>> =
            edges.into_iter().fold(HashMap::new(), |mut map, edge| {
                let (from, to) = (edge[0], edge[1]);
                map.entry(from).or_default().push(to);
                map.entry(to).or_default().push(from);
                map
            });

        let mut to_visit = vec![source];
        let mut visited = vec![false; n as usize];

        while let Some(next) = to_visit.pop() {
            if visited[next as usize] {
                continue;
            }
            if next == destination {
                return true;
            }
            if let Some(destinations) = edges.get(&next) {
                to_visit.extend(destinations);
            }
            visited[next as usize] = true;
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
