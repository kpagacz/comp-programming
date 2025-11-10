// https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/description/?envType=daily-question&envId=2025-11-10
struct Solution;

#[allow(dead_code)]
impl Solution {
    // I didn't realize I could use a monotonic stack here...
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut answer = 0;
        for num in nums {
            while stack
                .last()
                .is_some_and(|highest_so_far| *highest_so_far > num)
            {
                stack.pop();
            }
            if num == 0 {
                continue;
            }
            if stack
                .last()
                .is_none_or(|highest_so_far| num > *highest_so_far)
            {
                answer += 1;
                stack.push(num);
            }
        }
        answer
    }
}
fn main() {
    println!("Hello, world!");
}
