// https://leetcode.com/problems/sort-colors/description/
pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut zeros, mut ones) = (0, 0);
        for num in nums.iter() {
            match num {
                0 => zeros += 1,
                1 => ones += 1,
                _ => {}
            }
        }

        for num in nums {
            match (zeros > 0, ones > 0) {
                (true, _) => {
                    *num = 0;
                    zeros -= 1;
                }
                (false, true) => {
                    *num = 1;
                    ones -= 1;
                }
                (false, false) => *num = 2,
            }
        }
    }
}

fn main() {
    let test_cases = [vec![2, 0, 2, 1, 1, 0]];
    for mut nums in test_cases {
        Solution::sort_colors(&mut nums);
        println!("{:?}", nums);
    }
}
