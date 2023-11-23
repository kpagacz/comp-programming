// https://leetcode.com/problems/arithmetic-subarrays/description/
pub struct Solution {}

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut answer = Vec::with_capacity(l.len());

        for (&l, &r) in l.iter().zip(r.iter()) {
            let (l, r) = (l as usize, r as usize);
            let mut temp = Vec::with_capacity(r - l + 1);
            for &num in &nums[l..=r] {
                temp.push(num);
            }
            temp.sort_unstable();
            let delta = temp[1] - temp[0];
            answer.push(
                temp.windows(2)
                    .all(|adjacent| adjacent[1] - adjacent[0] == delta),
            );
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
