// https://leetcode.com/problems/robot-collisions/description/
pub struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        mut healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut robots_by_position = (0..positions.len()).collect::<Vec<_>>();
        robots_by_position.sort_unstable_by_key(|robot| positions[*robot]);
        let mut stack = vec![];
        let directions = directions.as_bytes();

        'outer: for robot in robots_by_position {
            while !stack.is_empty()
                && directions[robot] == b'L'
                && directions[*stack.last().unwrap()] == b'R'
            {
                let top_robot = stack.pop().unwrap();
                let top_health: i32 = healths[top_robot];
                let new_health = healths[robot];
                match top_health.cmp(&new_health) {
                    std::cmp::Ordering::Less => healths[robot] -= 1,
                    std::cmp::Ordering::Equal => continue 'outer,
                    std::cmp::Ordering::Greater => {
                        healths[top_robot] -= 1;
                        stack.push(top_robot);
                        continue 'outer;
                    }
                }
            }
            stack.push(robot);
        }

        stack.sort_unstable();
        stack.into_iter().map(|robot| healths[robot]).collect()
    }
}

fn main() {
    let test_cases = [(vec![3, 5, 2, 6], vec![10, 10, 15, 12], "RLRL".to_string())];
    for (positions, healths, directions) in test_cases {
        println!(
            "{:?}",
            Solution::survived_robots_healths(positions, healths, directions)
        );
    }
}
