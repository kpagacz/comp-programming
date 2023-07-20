// https://leetcode.com/problems/asteroid-collision/

pub struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        for asteroid in asteroids {
            if asteroid >= 0 {
                stack.push(asteroid);
            } else {
                loop {
                    match stack.last() {
                        Some(previous) if *previous >= 0 => match previous.cmp(&(-1 * asteroid)) {
                            std::cmp::Ordering::Less => stack.pop(),
                            std::cmp::Ordering::Equal => {
                                stack.pop();
                                break;
                            }
                            std::cmp::Ordering::Greater => break,
                        },
                        _ => {
                            stack.push(asteroid);
                            break;
                        }
                    };
                }
            }
        }
        stack
    }
}

fn main() {
    println!("Hello, world!");
}
