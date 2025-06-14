// https://leetcode.com/problems/transform-array-to-all-equal-elements/description/
pub struct Solution;

impl Solution {
    pub fn can_make_equal(nums: Vec<i32>, k: i32) -> bool {
        fn flips_to_make(nums: &[i32], target: i32) -> i32 {
            let mut next_flipped = false;
            let mut flips = 0;
            for i in 0..nums.len() - 1 {
                match (next_flipped, nums[i]) {
                    (false, t) if t == target => {}
                    (true, t) if t == target => {
                        flips += 1;
                    }
                    (false, t) if t != target => {
                        flips += 1;
                        next_flipped = true;
                    }
                    (true, t) if t != target => {
                        next_flipped = false;
                    }
                    _ => unreachable!(),
                }
            }

            if (*nums.last().unwrap() == target && !next_flipped)
                || (*nums.last().unwrap() != target && next_flipped)
            {
                flips
            } else {
                i32::MAX
            }
        }

        let to_minus_one = flips_to_make(&nums, -1);
        let to_one = flips_to_make(&nums, 1);

        k >= to_one.min(to_minus_one)
    }
}

fn main() {
    println!("Hello, world!");
}
