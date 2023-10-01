// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/
pub struct Solution {}
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut prefix_max = vec![i64::MIN; nums.len()];
        let mut suffix_max = vec![i64::MIN; nums.len()];
        prefix_max[0] = nums[0] as i64;
        *suffix_max.last_mut().unwrap() = *nums.last().unwrap() as i64;

        for i in 1..nums.len() {
            prefix_max[i] = prefix_max[i - 1].max(nums[i - 1] as i64);
        }
        for i in (0..(nums.len() - 1)).rev() {
            suffix_max[i] = suffix_max[i + 1].max(nums[i + 1] as i64);
        }

        let mut answer = i64::MIN;
        for i in 1..(nums.len() - 1) {
            answer = answer.max((prefix_max[i] - nums[i] as i64) * suffix_max[i]);
        }

        answer.max(0)
    }
}
fn main() {
    let test_cases = vec![vec![12, 6, 1, 2, 7], vec![1, 10, 3, 4, 19], vec![1, 2, 3]];
    for test in test_cases {
        println!("{}", Solution::maximum_triplet_value(test));
    }
}
