// https://leetcode.com/problems/redundant-connection/description/
pub struct Solution;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn find(&mut self, node: usize) -> usize {
        if self.parent[node] == usize::MAX {
            node
        } else {
            let root = self.find(self.parent[node]);
            self.parent[node] = root;
            root
        }
    }

    /// Returns whether first and second were already connected
    pub fn union(&mut self, first: usize, second: usize) -> bool {
        let first_root = self.find(first);
        let second_root = self.find(second);

        if first_root == second_root {
            true
        } else {
            self.parent[first_root] = second_root;
            false
        }
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind {
            parent: vec![usize::MAX; edges.len() + 1],
        };

        let mut redundant = vec![];
        for edge in edges {
            let (first, second) = (edge[0] as usize, edge[1] as usize);

            if uf.union(first, second) {
                redundant = edge;
            }
        }
        redundant
    }
}

fn main() {
    println!("Hello, world!");
}
