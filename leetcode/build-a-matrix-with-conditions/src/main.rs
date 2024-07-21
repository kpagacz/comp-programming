// https://leetcode.com/problems/build-a-matrix-with-conditions/description/
pub struct Solution;

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let row_sorted = Self::kahn(k as usize, &row_conditions);
        let col_sorted = Self::kahn(k as usize, &col_conditions);

        if row_sorted.is_empty() || col_sorted.is_empty() {
            return vec![];
        }

        let mut answer = vec![vec![0; k as usize]; k as usize];
        for i in 0..k as usize {
            for (j, &col) in col_sorted.iter().enumerate().take(k as usize) {
                if row_sorted[i] == col {
                    answer[i][j] = row_sorted[i];
                }
            }
        }
        answer
    }

    fn kahn(mut n: usize, edges: &[Vec<i32>]) -> Vec<i32> {
        let mut in_degrees = vec![0; n + 1];
        let mut adj = vec![vec![]; n + 1];

        for edge in edges {
            let [from, to]: [i32; 2] = edge[..2].try_into().unwrap();
            let (from, to) = (from as usize, to as usize);
            in_degrees[to] += 1;
            adj[from].push(to);
        }

        let mut answer = vec![];
        use std::collections::VecDeque;
        let mut queue = VecDeque::from_iter((1..=n).filter(|&node| in_degrees[node] == 0));

        while let Some(node) = queue.pop_front() {
            n -= 1;
            answer.push(node as i32);

            for &dest in &adj[node] {
                in_degrees[dest] -= 1;
                if in_degrees[dest] == 0 {
                    queue.push_back(dest);
                }
            }
        }

        if n == 0 {
            answer
        } else {
            vec![]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
