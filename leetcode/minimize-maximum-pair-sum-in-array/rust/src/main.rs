// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/
pub struct Solution {}

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut counting_sort = vec![0; 10usize.pow(5) + 1];
        nums.iter()
            .for_each(|num| counting_sort[*num as usize] += 1);

        let mut max = usize::MIN;

        let (mut left, mut right) = (0usize, counting_sort.len() - 1);
        loop {
            while left < counting_sort.len() && counting_sort[left] == 0 {
                left += 1;
            }
            while right > 0 && counting_sort[right] == 0 {
                right -= 1;
            }

            if left <= right {
                max = max.max(left + right);
                counting_sort[left] -= 1;
                counting_sort[right] -= 1;
            } else {
                break;
            }
        }
        max as i32
    }
    pub fn min_pair_sum_sort(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut max = i32::MIN;
        for i in 0..nums.len() / 2 {
            max = max.max(nums[i] + nums[nums.len() - 1 - i]);
        }
        max
    }
}

fn main() {
    println!("Hello, world!");
}
