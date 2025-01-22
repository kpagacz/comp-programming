// https://leetcode.com/problems/map-of-highest-peak/description/
pub struct Solution;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (is_water.len(), is_water[0].len());
        const NEIGHBOURS: [(usize, usize); 4] = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
        let mut map = vec![vec![-1; cols]; rows];
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        (0..rows).for_each(|row| {
            (0..cols).for_each(|col| {
                if is_water[row][col] == 1 {
                    queue.push_back((row, col));
                }
            })
        });

        let mut height = 0;
        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let (x, y) = queue.pop_front().unwrap();
                if map[x][y] != -1 {
                    continue;
                }
                map[x][y] = height;
                NEIGHBOURS
                    .into_iter()
                    .filter_map(|(dx, dy)| {
                        let (newx, newy) = (x + dx, y + dy);
                        if newx < rows && newy < cols && map[newx][newy] == -1 {
                            Some((newx, newy))
                        } else {
                            None
                        }
                    })
                    .for_each(|pos| queue.push_back(pos));
            }

            height += 1;
        }

        map
    }
}

fn main() {
    println!("Hello, world!");
}
