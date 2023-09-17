// https://leetcode.com/problems/shortest-path-visiting-all-nodes/
pub struct Solution {}

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        fn floyd_warshall(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut shortest_paths = vec![vec![i16::MAX as i32; graph.len()]; graph.len()];

            for i in 0..graph.len() {
                for &dest in &graph[i] {
                    shortest_paths[i][dest as usize] = 1;
                }
                shortest_paths[i][i] = 0;
            }

            for k in 0..shortest_paths.len() {
                for i in 0..shortest_paths.len() {
                    for j in 0..shortest_paths.len() {
                        shortest_paths[i][j] =
                            (shortest_paths[i][k] + shortest_paths[k][j]).min(shortest_paths[i][j]);
                    }
                }
            }

            shortest_paths
        }
        let shortest_paths = floyd_warshall(graph);

        use std::collections::HashMap;
        fn shortest_path_rec(
            total: i32,
            start_node: usize,
            visited_nodes: u16,
            paths: &Vec<Vec<i32>>,
            mem: &mut HashMap<(u16, usize), i32>,
            nodes_count: usize,
        ) -> i32 {
            // println!("{} {} {}", total, start_node, visited_nodes);
            if mem.contains_key(&(visited_nodes, start_node))
                && mem[&(visited_nodes, start_node)] <= total
            {
                return mem[&(visited_nodes, start_node)];
            }

            if visited_nodes == (1 << nodes_count) - 1 {
                mem.entry((visited_nodes, start_node))
                    .and_modify(|e| *e = (*e).min(total))
                    .or_insert(total);
                return total;
            }

            let mut min = i32::MAX;
            for new_start in 0..nodes_count {
                if (visited_nodes >> new_start) & 1 == 0 {
                    min = min.min(shortest_path_rec(
                        total + paths[start_node][new_start] as i32,
                        new_start,
                        visited_nodes | (1 << new_start),
                        paths,
                        mem,
                        nodes_count,
                    ));
                }
            }

            mem.entry((visited_nodes, start_node))
                .and_modify(|e| *e = (*e).min(min))
                .or_insert(min);

            min
        }

        let mut mem = HashMap::new();
        let mut min_path = i32::MAX;
        for start in 0..shortest_paths.len() {
            min_path = min_path.min(shortest_path_rec(
                0,
                start,
                1 << start,
                &shortest_paths,
                &mut mem,
                shortest_paths.len(),
            ));
        }

        min_path
    }
}

fn main() {
    let test_case = vec![vec![1, 2, 3], vec![0], vec![0], vec![0]];
    let test_case2 = vec![
        vec![1],
        vec![0, 2, 6],
        vec![1, 3],
        vec![2],
        vec![5],
        vec![4, 6],
        vec![1, 5, 7],
        vec![6],
    ];
    // println!("{:?}", Solution::shortest_path_length(test_case));
    // println!("{:?}", Solution::shortest_path_length(test_case2));
    let test_case3 = vec![
        vec![2, 3, 5, 7],
        vec![2, 3, 7],
        vec![0, 1],
        vec![0, 1],
        vec![7],
        vec![0],
        vec![10],
        vec![9, 10, 0, 1, 4],
        vec![9],
        vec![7, 8],
        vec![7, 6],
    ];
    println!("{:?}", Solution::shortest_path_length(test_case3));
}
