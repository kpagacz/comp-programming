// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/
pub struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(mut nums: Vec<i32>, limit: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut current_group = 0;
        let mut num_to_group = HashMap::new();
        use std::collections::VecDeque;
        let mut groups = vec![VecDeque::new()];

        let mut copy = nums.clone();
        copy.sort_unstable();

        let mut previous = copy[0];
        for num in copy {
            if num - previous > limit {
                current_group += 1;
                groups.push(VecDeque::new());
            }
            num_to_group.insert(num, current_group);
            groups[current_group].push_back(num);
            previous = num;
        }

        nums.iter_mut().for_each(|num| {
            let group = *num_to_group.get(num).unwrap();
            *num = groups[group].pop_front().unwrap();
        });
        nums
    }
}

fn main() {
    println!("Hello, world!");
}
