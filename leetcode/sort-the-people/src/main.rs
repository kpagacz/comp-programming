// https://leetcode.com/problems/sort-the-people/description/?envType=daily-question&envId=2024-07-22
pub struct Solution;

impl Solution {
    pub fn sort_people(mut names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people: Vec<_> = (0..names.len()).collect();
        people.sort_unstable_by_key(|&person| -heights[person]);
        people
            .into_iter()
            .map(|person| std::mem::replace(&mut names[person], "".to_string()))
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
