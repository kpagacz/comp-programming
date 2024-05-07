// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/description/
pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut appeared = vec![false; nums.len() + 1];

        for num in nums {
            appeared[num as usize] = true;
        }

        appeared
            .into_iter()
            .skip(1)
            .enumerate()
            .filter_map(|(pos, appeared)| if appeared { None } else { Some(1 + pos as i32) })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
