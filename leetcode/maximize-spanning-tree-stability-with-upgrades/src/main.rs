// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades/description/?envType=daily-question&envId=2026-03-12
struct Solution;

#[derive(Debug)]
struct UnionFind {
    parents: Vec<usize>,
    heights: Vec<u32>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            heights: vec![0; size],
        }
    }

    fn find(&mut self, node: usize) -> usize {
        let mut it = node;
        while self.parents[it] != it {
            it = self.parents[it];
        }
        self.parents[node] = it;
        it
    }

    fn merge(&mut self, first: usize, second: usize) -> bool {
        let first_root = self.find(first);
        let second_root = self.find(second);

        if first_root != second_root {
            if self.heights[first_root] >= self.heights[second_root] {
                self.parents[second_root] = first_root;
            } else {
                self.parents[first_root] = second_root;
            }
            true
        } else {
            false
        }
    }

    fn is_spanning(&mut self) -> bool {
        let first = self.find(0);
        (0..self.parents.len()).all(|node| first == self.find(node))
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        // Use DSU
        // Initialize the DSU with edges that must be in
        // Track the strength of included edges in a PQ
        // Sort the edges starting from the most strength
        // Keep adding edges, tracking the strength of added edges
        // Find the min strength edge included (from the pq)
        // If it's must - return must
        // If it's not a must return the second lowest
        //
        // This certainly constructs a valid spanning tree
        // Does sorting guarantee that the stability is maximized?
        // Proof by contradiction. If it doesn't that there was
        // a situation during adding the edges where we already had
        // an edge between sets and the newly added edge is greater
        // strength than what we connected the sets with. But this
        // contradicts the assumption that we process the edges
        // from highest str to lowest str

        const SOURCE: usize = 0;
        const DEST: usize = 1;
        const STR: usize = 2;
        const MUST: usize = 3;
        use std::collections::BinaryHeap;
        let mut must_edges = BinaryHeap::from_iter(
            edges
                .iter()
                .enumerate()
                .filter(|(_, edge)| edge[MUST] == 1)
                .map(|(pos, edge)| (edge[STR], edge[SOURCE] as usize, edge[DEST] as usize, pos)),
        );
        let mut optional_edges = BinaryHeap::from_iter(
            edges
                .iter()
                .enumerate()
                .filter(|(_, edge)| edge[MUST] == 0)
                .map(|(pos, edge)| (edge[STR], edge[SOURCE] as usize, edge[DEST] as usize, pos)),
        );

        let mut added_edges = BinaryHeap::new(); // (str, must) MIN HEAP
        let mut uf = UnionFind::new(n as usize);
        // Add must edges
        while let Some((str, from, to, pos)) = must_edges.pop() {
            if !uf.merge(from, to) {
                return -1;
            }
            added_edges.push(std::cmp::Reverse((str, 1, pos)));
        }

        // Add optional edges
        while let Some((str, from, to, pos)) = optional_edges.pop() {
            if uf.merge(from, to) {
                added_edges.push(std::cmp::Reverse((str, 0, pos)));
            }
        }

        // Check if all parents are the same - if not, then
        // spanning tree is impossible
        if !uf.is_spanning() {
            return -1;
        }

        // Perform k upgrades
        let mut was_upgraded = vec![false; edges.len()];
        for _ in 0..k {
            if let Some(std::cmp::Reverse((str, must, pos))) = added_edges.pop() {
                if was_upgraded[pos] {
                    added_edges.push(std::cmp::Reverse((str, must, pos)));
                } else {
                    let new_str = if must == 1 { str } else { 2 * str };
                    was_upgraded[pos] = true;
                    added_edges.push(std::cmp::Reverse((new_str, must, pos)));
                }
            }
        }

        added_edges.pop().unwrap().0.0
    }
}

fn main() {
    let test_cases = [
        (3, vec![vec![0, 1, 2, 1], vec![1, 2, 3, 0]], 1, 2),
        (
            3,
            vec![vec![0, 1, 4, 0], vec![1, 2, 3, 0], vec![0, 2, 1, 0]],
            2,
            6,
        ),
        (
            3,
            vec![vec![0, 1, 1, 1], vec![1, 2, 1, 1], vec![2, 0, 1, 1]],
            0,
            -1,
        ),
        (2, vec![vec![0, 1, 100000, 0]], 1, 200000),
    ];

    for (n, edges, k, exp) in test_cases {
        println!("Edges: {edges:?}");
        println!("{} exp: {exp}", Solution::max_stability(n, edges, k));
    }
}
