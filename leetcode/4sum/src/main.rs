// https://leetcode.com/problems/4sum/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        type FreqMap = HashSet<i64>;
        fn k_sum(k: usize, target: i64, nums: &[i32]) -> Vec<Vec<i32>> {
            let mut answer = vec![];
            if k == 2 {
                let mut freq = FreqMap::new();
                for num in nums {
                    if freq.contains(&(target - *num as i64)) {
                        answer.push(vec![target as i32 - num, *num]);
                    }
                    freq.insert(*num as i64);
                }
            } else {
                for i in 0..nums.len() {
                    let mut partial_results = k_sum(k - 1, target - nums[i] as i64, &nums[i + 1..]);
                    partial_results.iter_mut().for_each(|arr| arr.push(nums[i]));
                    answer.extend(partial_results);
                }
            }
            answer
        }

        nums.sort_unstable();
        let mut res = k_sum(4, target as i64, &nums);
        res.sort_unstable();
        res.dedup();
        res
    }
}

fn main() {
    println!("Hello, world!");
}
