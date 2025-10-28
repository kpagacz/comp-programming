// https://leetcode.com/problems/make-array-elements-equal-to-zero/description/?envType=daily-question&envId=2025-10-28
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let mut answer = 0;
        let mut current = 0;
        for num in nums {
            current += num;
            if num == 0 {
                if 2 * current == sum {
                    answer += 2;
                } else if (sum - 2 * current).abs() == 1 {
                    answer += 1;
                }
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
