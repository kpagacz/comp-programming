// https://leetcode.com/problems/walking-robot-simulation/description/
pub struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        use std::collections::HashSet;
        let obstacles = obstacles
            .into_iter()
            .fold(HashSet::new(), |mut set, obstacle| {
                set.insert((obstacle[0], obstacle[1]));
                set
            });

        let mut dir = 1;
        let mut pos = (0, 0);
        let mut max_distance = 0;

        for command in commands {
            match command {
                -1 => dir = (dir + 1) % DIRECTIONS.len(),
                -2 => dir = (dir + DIRECTIONS.len() - 1) % DIRECTIONS.len(),
                1..=9 => {
                    let delta = DIRECTIONS[dir];
                    for _ in 0..command {
                        let new_pos = (pos.0 + delta.0, pos.1 + delta.1);
                        if obstacles.contains(&new_pos) {
                            continue;
                        } else {
                            pos = new_pos;
                            max_distance = max_distance.max(pos.0.pow(2) + pos.1.pow(2));
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        max_distance
    }
}

fn main() {
    println!("Hello, world!");
}
