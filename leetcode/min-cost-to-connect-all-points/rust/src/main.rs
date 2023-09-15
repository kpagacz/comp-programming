// https://leetcode.com/problems/min-cost-to-connect-all-points/
pub struct Solution {}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
pub struct Edge {
    dist: i32,
    source: usize,
    dest: usize,
}

impl Edge {
    pub fn new(dist: i32, source: usize, dest: usize) -> Self {
        Self { dist, source, dest }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    pub fn new(length: usize) -> Self {
        Self {
            parents: (0..length).collect(),
        }
    }

    pub fn find(&mut self, node: usize) -> usize {
        if self.parents[node as usize] == node {
            return node;
        } else {
            self.parents[node] = self.find(self.parents[node]);
            self.parents[node]
        }
    }

    pub fn join(&mut self, a: usize, b: usize) {
        let a_root = self.find(a);
        let b_root = self.find(b);

        if a_root != b_root {
            self.parents[a_root] = b_root;
        }
    }
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        let mut edge_pqueue = std::collections::BinaryHeap::new();
        let distances = Self::calculate_distances(&points);

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                edge_pqueue.push(std::cmp::Reverse(Edge::new(distances[i][j], i, j)));
            }
        }

        let mut added_edges = 0;
        let mut uf = UnionFind::new(points.len());

        while added_edges != points.len() - 1 {
            let edge = edge_pqueue.pop().unwrap().0;
            let source_root = uf.find(edge.source);
            let dest_root = uf.find(edge.dest);

            if source_root != dest_root {
                answer += edge.dist;
                added_edges += 1;
                uf.join(source_root, dest_root);
            }
        }

        answer
    }

    fn calculate_distances(points: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = vec![vec![0; points.len()]; points.len()];
        for i in 0..points.len() {
            for j in i..points.len() {
                answer[i][j] = Self::distance(&points[i], &points[j]);
            }
        }
        answer
    }

    fn distance(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
        (b[0] - a[0]).abs() + (b[1] - a[1]).abs()
    }
}

fn main() {
    let test = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
    let test2 = vec![vec![2, -3], vec![-17, -8], vec![13, 8], vec![-17, -15]];
    println!("{}", Solution::min_cost_connect_points(test2));
}
