// https://leetcode.com/problems/minimum-cost-walk-in-weighted-graph/solutions/?envType=daily-question&envId=2025-03-20
pub struct Solution;

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parents: (0..n).collect(),
        }
    }

    fn find(&mut self, n: usize) -> usize {
        if self.parents[n] == n {
            n
        } else {
            let root = self.find(self.parents[n]);
            self.parents[n] = root;
            root
        }
    }

    fn merge(&mut self, first: usize, second: usize) {
        let first_root = self.find(first);
        let second_root = self.find(second);

        if first_root != second_root {
            self.parents[second_root] = first_root;
        }
    }
}

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(n as usize);

        let mut component_walk_costs = vec![i32::MAX; n as usize];

        for edge in edges {
            let (first, second, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
            let first_root = uf.find(first);
            let second_root = uf.find(second);
            component_walk_costs[first_root] &= component_walk_costs[second_root] & cost;
            uf.merge(first, second);
        }

        query
            .into_iter()
            .map(|query| {
                let (first, second) = (query[0] as usize, query[1] as usize);
                if uf.find(first) == uf.find(second) {
                    let component = uf.find(first);
                    component_walk_costs[component]
                } else {
                    -1
                }
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
