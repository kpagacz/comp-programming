// https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/description/
pub struct Solution;

use std::collections::{BinaryHeap, HashMap, HashSet};

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len() - 1, grid[0].len() - 1);
        let graph = Solution::build_graph(grid);
        let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::from_iter(vec![(0, 0, 0)]);
        let mut visited: HashSet<(i32, usize, usize)> = HashSet::new();
        while !heap.is_empty() {
            let (mut total_cost, row, col) = heap.pop().unwrap();
            total_cost = -total_cost;
            if row == rows && col == cols {
                return total_cost;
            }
            for &(mut cost, row, col) in &graph[&(row, col)] {
                cost = -cost;
                if visited.contains(&(cost, row, col)) {
                    continue;
                };
                visited.insert((cost, row, col));
                heap.push((-(total_cost + cost), row, col));
            }
        }
        0
    }

    fn build_graph(grid: Vec<Vec<i32>>) -> HashMap<(usize, usize), Vec<(i32, usize, usize)>> {
        let mut result = HashMap::new();
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let dir = grid[row][col];
                let mut neighs = vec![];
                if row > 0 {
                    neighs.push((if dir == 4 { 0 } else { -1 }, row - 1, col));
                }
                if col > 0 {
                    neighs.push((if dir == 2 { 0 } else { -1 }, row, col - 1));
                }
                if row < grid.len() - 1 {
                    neighs.push((if dir == 3 { 0 } else { -1 }, row + 1, col));
                }
                if col < grid[row].len() - 1 {
                    neighs.push((if dir == 1 { 0 } else { -1 }, row, col + 1));
                }
                result.insert((row, col), neighs);
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
