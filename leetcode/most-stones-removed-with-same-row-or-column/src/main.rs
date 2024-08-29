// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/
pub struct Solution;

struct UnionFind {
    arr: Vec<usize>,
}
impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            arr: (0..size).collect(),
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.arr[node] == node {
            node
        } else {
            let root = self.find(self.arr[node]);
            self.arr[node] = root;
            root
        }
    }

    fn union(&mut self, first: usize, second: usize) {
        let first_root = self.find(first);
        let second_root = self.find(second);

        if first_root != second_root {
            self.arr[first_root] = second_root;
        }
    }

    fn unions_counts(&mut self) -> Vec<i32> {
        for i in 0..self.arr.len() {
            self.find(i);
        }

        let mut counts = vec![0; self.arr.len()];
        for root in &self.arr {
            counts[*root] += 1;
        }
        counts.into_iter().filter(|count| *count != 0).collect()
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut union = UnionFind::new(stones.len());
        use std::collections::HashMap;
        let (row_map, col_map) = stones.iter().enumerate().fold(
            (HashMap::new(), HashMap::new()),
            |(mut row_map, mut col_map), (pos, stone)| {
                let (row, col) = (stone[0], stone[1]);
                row_map.entry(row).or_insert(Vec::new()).push(pos);
                col_map.entry(col).or_insert(Vec::new()).push(pos);
                (row_map, col_map)
            },
        );

        for (_, row_stones) in row_map {
            let last_stone = *row_stones.last().unwrap();
            for stone in row_stones {
                union.union(stone, last_stone);
            }
        }
        for (_, col_stones) in col_map {
            let last_stone = *col_stones.last().unwrap();
            for stone in col_stones {
                union.union(stone, last_stone);
            }
        }

        union
            .unions_counts()
            .iter()
            .map(|count| count - 1)
            .sum::<i32>()
    }
}

fn main() {
    let test_cases = [vec![
        vec![0, 1],
        vec![1, 2],
        vec![1, 3],
        vec![3, 3],
        vec![2, 3],
        vec![0, 2],
    ]];

    for stones in test_cases {
        println!("{}", Solution::remove_stones(stones));
    }
}
