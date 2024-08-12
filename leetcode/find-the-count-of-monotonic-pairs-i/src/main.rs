// https://leetcode.com/problems/find-the-count-of-monotonic-pairs-i/description/
pub struct Solution;

const NUM_MAX: usize = 50;
const MODULO: i32 = 10i32.pow(9) + 7;
impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0i32; NUM_MAX + 1];
        dp.iter_mut().enumerate().for_each(|(pos, count)| {
            *count = if pos as i32 > nums[0] { 0 } else { 1 };
        });

        for i in 1..nums.len() {
            let prefix_sum = dp
                .iter()
                .scan(0i32, |sum, num| {
                    *sum = (*sum + num) % MODULO;
                    Some(*sum)
                })
                .collect::<Vec<_>>();
            dp.iter_mut().enumerate().for_each(|(pos, count)| {
                if nums[i - 1] - nums[i] + (pos as i32) < 0 || pos as i32 > nums[i] {
                    *count = 0;
                } else {
                    *count =
                        prefix_sum[usize::min(pos, (nums[i - 1] - nums[i] + pos as i32) as usize)];
                }
            });
        }

        dp.into_iter()
            .fold(0i32, |acc, count| (acc + count) % MODULO)
    }
}

fn main() {
    let test_cases = [vec![2, 3, 2]];
    for nums in test_cases {
        println!("{}", Solution::count_of_pairs(nums));
    }
}
