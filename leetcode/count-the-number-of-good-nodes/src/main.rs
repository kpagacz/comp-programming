// https://leetcode.com/problems/count-the-number-of-good-nodes/description/
pub struct Solution;

impl Solution {
    pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; 100_001];
        for mut edge in edges {
            edge.sort_unstable();
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }

        Self::rec(0, -1, &graph).1
    }

    fn rec(node: usize, parent: i32, graph: &[Vec<i32>]) -> (i32, i32) {
        if graph[node].len() == 1 && node != 0 {
            return (1, 1); // size, number of good nodes
        }

        let mut answer = (1, 0);

        let mut children_sizes = vec![];
        for &dest in &graph[node] {
            if dest == parent {
                continue;
            }
            let (subtree_size, good_nodes) = Self::rec(dest as usize, node as i32, graph);
            children_sizes.push(subtree_size);
            answer.0 += subtree_size;
            answer.1 += good_nodes;
        }

        let first_child_size = *children_sizes.first().unwrap();
        if children_sizes
            .iter()
            .all(|&child_size| child_size == first_child_size)
        {
            answer.1 += 1;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
