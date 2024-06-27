// https://leetcode.com/problems/find-center-of-star-graph/description/
pub struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let first = &edges[0];
        let second = &edges[1];

        match (first[0] == second[0], first[0] == second[1]) {
            (true, false) => first[0],
            (true, true) => unreachable!(),
            (false, true) => first[0],
            (false, false) => first[1],
        }
    }
}

fn main() {
    println!("Hello, world!");
}
