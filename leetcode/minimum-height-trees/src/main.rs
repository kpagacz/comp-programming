// https://leetcode.com/problems/minimum-height-trees/description/
pub struct Solution;

impl Solution {
    pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![Vec::new(); n as usize];
        let mut degrees = vec![0; n as usize];
        edges.iter().for_each(|edge| {
            let (first, second) = (edge[0] as usize, edge[1] as usize);
            graph[first].push(second);
            graph[second].push(first);
            degrees[first] += 1;
            degrees[second] += 1;
        });

        let mut leaves = degrees
            .iter()
            .enumerate()
            .filter_map(|(id, &degree)| if degree <= 1 { Some(id) } else { None })
            .collect::<Vec<_>>();
        while n > 2 {
            let mut new_leaves = vec![];

            while let Some(leave) = leaves.pop() {
                for &node in &graph[leave] {
                    degrees[node] -= 1;
                    if degrees[node] == 1 {
                        new_leaves.push(node);
                    }
                }
                n -= 1;
            }

            leaves = new_leaves;
        }

        leaves.into_iter().map(|leave| leave as i32).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
