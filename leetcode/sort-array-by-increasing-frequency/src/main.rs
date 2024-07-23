// https://leetcode.com/problems/sort-array-by-increasing-frequency/description/
pub struct Solution;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let mut freq = [0; 201];

        for &num in &nums {
            freq[num as usize + 100] += 1;
        }

        nums.sort_by(|&a, &b| {
            freq[a as usize + 100]
                .cmp(&freq[b as usize + 100])
                .then(b.cmp(&a))
        });

        nums
    }
}

fn main() {
    println!("Hello, world!");
}
