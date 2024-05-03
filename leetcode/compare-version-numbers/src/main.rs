// https://leetcode.com/problems/compare-version-numbers/description/
pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut version1 = version1
            .as_str()
            .split('.')
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut version2 = version2
            .as_str()
            .split('.')
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        while version1.len() < version2.len() {
            version1.push(0);
        }
        while version1.len() > version2.len() {
            version2.push(0);
        }
        match version1.cmp(&version2) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
