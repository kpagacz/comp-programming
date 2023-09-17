// https://leetcode.com/problems/shortest-path-visiting-all-nodes/
pub struct Solution {}

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let mut bfs = std::collections::VecDeque::new();
        let mut seen_states = std::collections::HashSet::new();

        for start_node in 0..graph.len() {
            bfs.push_back(((1 << start_node) * 16 + start_node, 0));
        }

        while let Some(state) = bfs.pop_front() {
            if !seen_states.contains(&state.0) {
                let visited_mask = state.0 >> 4;
                let current_node = state.0 & ((1 << 4) - 1);
                // println!("{}", format!("{visited_mask:#016b}"));
                let dist = state.1;

                if visited_mask == (1 << graph.len()) - 1 {
                    return dist;
                }

                for &new_current in &graph[current_node] {
                    let new_dest = new_current as usize;
                    bfs.push_back(((visited_mask | (1 << new_dest)) * 16 + new_dest, dist + 1));
                }

                seen_states.insert(state.0);
            }
        }

        -1
    }
}

fn main() {
    let test_case = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
    println!("{:?}", Solution::shortest_path_length(test_case));
}
