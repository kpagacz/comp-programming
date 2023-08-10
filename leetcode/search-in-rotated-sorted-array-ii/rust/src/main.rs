// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/description/
pub struct Solution {}

impl Solution {
    // Runtime 1ms Beats 84.00%of users with Rust
    // Memory 2.07mb Beats 72.00%of users with Rust
    pub fn search(mut nums: Vec<i32>, target: i32) -> bool {
        nums.dedup();
        while nums.first().unwrap() == nums.last().unwrap() && nums.len() > 1 {
            nums.pop();
        }
        let rotation_point = Self::find_rotation_point(&nums);
        nums[..rotation_point].binary_search(&target).is_ok()
            || nums[rotation_point..].binary_search(&target).is_ok()
    }

    fn find_rotation_point(nums: &Vec<i32>) -> usize {
        let (mut left, mut right) = (0_usize, nums.len() - 1);

        while left < right {
            let middle = (left + right) / 2;

            match (nums[middle] > nums[middle + 1], nums[middle] > nums[0]) {
                (true, _) => return middle + 1,
                (_, true) => left = middle + 1,
                (_, false) => right = middle,
            }
        }

        right
    }
}

fn main() {
    println!("Hello, world!");
}
