// https://leetcode.com/problems/find-minimum-time-to-reach-last-room-ii/description/?envType=daily-question&envId=2025-05-08
pub struct Solution;

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::new();
        let rows = move_time.len();
        let cols = move_time[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        pq.push((0, 0, 0, true)); // -time, x, y

        while let Some((neg_time, x, y, costs_one)) = pq.pop() {
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

            let additional_cost = if costs_one { 1 } else { 2 };
            let costs_one = !costs_one;
            for (delta_x, delta_y) in NEIGHBOURS {
                let new_x = x.wrapping_add(delta_x);
                let new_y = y.wrapping_add(delta_y);

                if new_x < rows && new_y < cols {
                    let new_time =
                        (time + additional_cost).max(move_time[new_x][new_y] + additional_cost);
                    pq.push((-new_time, new_x, new_y, costs_one));
                }
            }
        }

        -1
    }
}

fn main() {
    println!("Hello, world!");
}
