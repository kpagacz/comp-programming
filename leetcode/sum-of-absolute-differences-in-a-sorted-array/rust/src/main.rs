// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/
pub struct Solution {}
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let prefix_sum: Vec<_> = nums
            .iter()
            .scan(0, |sum, num| {
                *sum += num;
                Some(*sum)
            })
            .collect();
        let mut sufix_sum: Vec<_> = nums
            .iter()
            .rev()
            .scan(0, |sum, num| {
                *sum += num;
                Some(*sum)
            })
            .into_iter()
            .collect();
        sufix_sum = sufix_sum.into_iter().rev().collect();

        let n = nums.len();
        nums.into_iter()
            .enumerate()
            .map(|(i, num)| {
                (i + 1) as i32 * num - prefix_sum[i] + sufix_sum[i] - (n - i) as i32 * num
            })
            .collect()
    }
}

fn main() {
    let test_cases = vec![vec![1, 4, 6, 8, 10], vec![1]];
    for test in test_cases {
        println!("{:?}", Solution::get_sum_absolute_differences(test));
    }
}
