// https://leetcode.com/problems/predict-the-winner/
pub struct Solution {}

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let mut mem = std::collections::HashMap::new();
        let sum = nums.iter().sum::<i32>();
        Self::calculate_max_score(&mut mem, &nums, 0, nums.len() - 1, sum);

        let max_score = mem.get(&(0, nums.len() - 1)).unwrap();
        sum - *max_score <= *max_score
    }

    fn calculate_max_score(
        mem: &mut std::collections::HashMap<(usize, usize), i32>,
        nums: &Vec<i32>,
        start: usize,
        end: usize,
        score_left: i32,
    ) -> i32 {
        if mem.contains_key(&(start, end)) {
            return mem[&(start, end)];
        }
        if start == end {
            mem.entry((start, end)).or_insert(nums[start]);
            return nums[start];
        }
        let max_from_here = std::cmp::max(
            score_left
                - Self::calculate_max_score(mem, nums, start + 1, end, score_left - nums[start]),
            score_left
                - Self::calculate_max_score(mem, nums, start, end - 1, score_left - nums[end]),
        );
        mem.entry((start, end)).or_insert(max_from_here);
        mem[&(start, end)]
    }
}

fn main() {
    println!("Hello, world!");
}
