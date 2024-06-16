// https://leetcode.com/problems/patching-array/
pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut max_sum = 0;
        let mut answer = 0;

        for num in nums {
            if num > n {
                break;
            }

            while max_sum + 1 < num {
                answer += 1;
                max_sum = 2 * max_sum + 1;
            }
            max_sum += num;
        }
        while max_sum < n {
            answer += 1;
            max_sum = max_sum.saturating_add(max_sum).saturating_add(1);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
