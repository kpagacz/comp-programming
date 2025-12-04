// https://leetcode.com/problems/count-collisions-on-a-road/description/?envType=daily-question&envId=2025-12-04
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut stack = vec![];
        let mut collisions = 0;
        for c in directions.as_bytes() {
            match c {
                b'L' => {
                    let old_collisions = collisions;
                    if let Some(last) = stack.pop() {
                        match last {
                            b'S' => {
                                collisions += 1;
                            }
                            b'R' => {
                                collisions += 2;
                            }
                            _ => {}
                        }
                    }
                    while let Some(last) = stack.pop()
                        && last == b'R'
                    {
                        collisions += 1;
                    }
                    if old_collisions != collisions {
                        stack.push(b'S');
                    }
                }
                b'R' => {
                    while !stack.is_empty() && stack.last().unwrap() != &b'R' {
                        stack.pop();
                    }
                    stack.push(b'R');
                }
                b'S' => {
                    while let Some(last) = stack.pop() {
                        if last == b'R' {
                            collisions += 1;
                        }
                    }
                    stack.push(b'S');
                }
                _ => unreachable!(),
            }
        }
        collisions
    }
}

fn main() {
    let test_cases = [("RLRSLL", 5), ("LLRR", 0), ("LLLSLRLLSRLSLR", 7)];
    for (directions, exp) in test_cases {
        println!(
            "{} exp: {exp}",
            Solution::count_collisions(directions.to_owned())
        );
    }
}
