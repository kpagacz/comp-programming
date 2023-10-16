// https://leetcode.com/problems/find-indices-with-index-and-value-difference-ii/
pub struct Solution {}

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        use std::collections::BTreeSet;
        let mut seen = BTreeSet::new();
        for (id, &num) in nums.iter().enumerate() {
            if id as i32 - index_difference >= 0 {
                seen.insert((nums[id - index_difference as usize], id - index_difference as usize));
            }
            if seen.len() > 0 {
                match (
                    seen.iter().next().unwrap().0 <= num - value_difference,
                    seen.iter().next_back().unwrap().0 >= num + value_difference,
                ) {
                    (true, _) => return vec![seen.iter().next().unwrap().1 as i32, id as i32],
                    (_, true) => return vec![seen.iter().next_back().unwrap().1 as i32, id as i32],
                    (_, _) => {}
                }
            }
        }

        println!("{seen:?}");
        vec![-1, -1]
    }
}

fn main() {
    println!("Hello, world!");
}
