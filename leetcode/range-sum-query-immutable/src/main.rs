// https://leetcode.com/problems/range-sum-query-immutable/description/

struct NumArray {
    prefix: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        nums.splice(0..0, std::iter::once(0));
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        Self { prefix: nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix[right as usize + 1] - self.prefix[left as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

fn main() {
    println!("Hello, world!");
}
