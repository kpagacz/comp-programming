// https://leetcode.com/problems/build-an-array-with-stack-operations/description/
pub struct Solution {}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut target_it = 0_usize;
        (1..=n)
            .flat_map(|num| {
                if target_it == target.len() {
                    vec![]
                } else if target[target_it] == num {
                    target_it += 1;
                    vec![String::from("Push")]
                } else {
                    vec![String::from("Push"), String::from("Pop")]
                }
            })
            .collect()
    }
}
fn main() {
    println!("Hello, world!");
}
