// https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable/description/
pub struct Solution;

#[derive(Debug)]
struct UnionFind {
    nodes: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![usize::MAX; n + 1],
        }
    }

    fn root(&mut self, n: usize) -> usize {
        if self.nodes[n] == usize::MAX {
            n
        } else {
            self.nodes[n] = self.root(self.nodes[n]);
            self.nodes[n]
        }
    }

    fn join(&mut self, first: usize, second: usize) -> bool {
        let first_root = self.root(first);
        let second_root = self.root(second);

        let different_root = first_root != second_root;
        if different_root {
            self.nodes[first_root] = second_root;
        }
        different_root
    }

    fn all_nodes_reachable(&self) -> bool {
        self.nodes
            .iter()
            .filter(|&&root| root == usize::MAX)
            .count()
            == 2
    }
}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        let original = edges.len();

        edges.sort_unstable_by_key(|edge| -edge[0]);

        // MST for two graphs counting used edges
        let mut used = 0;

        let mut alice_mst = UnionFind::new(n as usize);
        let mut bob_mst = UnionFind::new(n as usize);
        for edge in edges {
            let (edge_type, from, to) = (edge[0], edge[1] as usize, edge[2] as usize);
            match edge_type {
                1 => {
                    if alice_mst.join(from, to) {
                        used += 1;
                    }
                }
                2 => {
                    if bob_mst.join(from, to) {
                        used += 1;
                    }
                }
                _ => {
                    let alice_needs_it = alice_mst.join(from, to);
                    let bob_needs_it = bob_mst.join(from, to);
                    if alice_needs_it || bob_needs_it {
                        used += 1;
                    }
                }
            }
        }

        if alice_mst.all_nodes_reachable() && bob_mst.all_nodes_reachable() {
            (original - used) as _
        } else {
            -1
        }
    }
}

fn main() {
    println!("Hello, world!");
}
