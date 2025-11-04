// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/description/?envType=daily-question&envId=2025-11-04
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        let x = x as usize;
        let k = k as usize;
        let calculate_x_sum = |arr: &[i32]| {
            let mut freqs = HashMap::new();
            for &num in arr {
                freqs.entry(num).and_modify(|freq| *freq += 1).or_insert(1);
            }
            let mut pq = BinaryHeap::new();
            for (key, value) in freqs.iter() {
                pq.push((value, key));
            }
            let mut answer = 0;
            for _ in 0..x {
                if let Some((freq, key)) = pq.pop() {
                    answer += key * freq;
                }
            }
            answer
        };

        let mut answer = Vec::with_capacity(nums.len() - k + 1);
        for i in 0..=nums.len() - k {
            answer.push(calculate_x_sum(&nums[i..(i + k)]));
        }
        answer
    }
}

fn main() {
    let test_cases = [(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2)];

    for (nums, k, x) in test_cases {
        println!("{:?}", Solution::find_x_sum(nums, k, x));
    }
}
