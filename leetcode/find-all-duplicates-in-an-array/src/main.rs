// https://leetcode.com/problems/find-all-duplicates-in-an-array/
pub struct Solution;

// That's a neat trick here. You can always reuse a given array to add
// O(n) info because you can use the sign to signify something.

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![];

        for i in 0..nums.len() {
            let pos = nums[i].unsigned_abs() as usize - 1;
            if nums[pos] < 0 {
                answer.push(nums[i].abs());
            } else {
                nums[pos] = -nums[pos];
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
