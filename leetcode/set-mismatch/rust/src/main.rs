// https://leetcode.com/problems/set-mismatch/description/?envType=daily-question&envId=2024-01-22
pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::BTreeMap;
        let len = nums.len() as i32;
        let nums = nums.into_iter().fold(BTreeMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });
        let mut answer = vec![];
        answer.push(*nums.iter().find(|&entry| entry.1 == &2).unwrap().0);

        for i in 1..len + 1 {
            if !nums.contains_key(&i) {
                answer.push(i);
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
