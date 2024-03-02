// https://leetcode.com/problems/squares-of-a-sorted-array/description/
pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut begin = 0;
        let mut end = nums.len() as i32 - 1;
        let mut answer = vec![];
        while begin <= end {
            if nums[begin as usize].abs() <= nums[end as usize].abs() {
                answer.push(nums[end as usize] * nums[end as usize]);
                end -= 1;
            } else {
                answer.push(nums[begin as usize] * nums[begin as usize]);
                begin += 1;
            }
        }
        answer.reverse();
        answer
    }
}

fn main() {
    let tests = vec![
        vec![-4, -1, 0, 3, 10],
        vec![-7, -3, 2, 3, 11],
        vec![-20, -7, -3, 2, 3, 11],
    ];
    for nums in tests {
        println!("{:?}", Solution::sorted_squares(nums));
    }
}
