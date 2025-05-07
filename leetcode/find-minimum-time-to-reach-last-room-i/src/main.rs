// https://leetcode.com/problems/find-minimum-time-to-reach-last-room-i/description/?envType=daily-question&envId=2025-05-07
pub struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::new();
        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        pq.push((0, 0, 0)); // -time, x, y

        while let Some((neg_time, x, y)) = pq.pop() {
            let time = -neg_time;
            if visited[x][y] {
                continue;
            }
            if x == rows - 1 && y == cols - 1 {
                return time;
            }
            visited[x][y] = true;

            const NEIGHBOURS: [(usize, usize); 4] =
                [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];

            for (delta_x, delta_y) in NEIGHBOURS {
                let new_x = x.wrapping_add(delta_x);
                let new_y = y.wrapping_add(delta_y);

                if new_x < rows && new_y < cols {
                    let new_time = (time + 1).max(move_time[new_x][new_y] + 1);
                    pq.push((-new_time, new_x, new_y));
                }
            }
        }

        -1
    }
}

fn main() {
    let test_cases = [
        vec![vec![0, 4], vec![4, 4]],
        vec![vec![0, 0, 0], vec![0, 0, 0]],
    ];
    for move_time in test_cases {
        println!("{}", Solution::min_time_to_reach(move_time));
    }
}
