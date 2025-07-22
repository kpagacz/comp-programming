// https://leetcode.com/problems/maximum-erasure-value/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut freqs = vec![0; 10usize.pow(4) + 1];

        let mut left = 0;
        let mut answer = 0;
        let mut curr = 0;

        for (i, &num) in nums.iter().enumerate() {
            curr += num;
            freqs[num as usize] += 1;

            while left < i && freqs[num as usize] > 1 {
                curr -= nums[left];
                freqs[nums[left] as usize] -= 1;
                left += 1;
            }

            answer = answer.max(curr);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
