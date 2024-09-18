// https://leetcode.com/problems/largest-number/description/
pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<_> = nums.iter().map(i32::to_string).collect();

        nums.sort_unstable_by(|a, b| {
            let ab = format!("{a}{b}");
            let ba = format!("{b}{a}");
            ba.cmp(&ab)
        });

        if nums[0].starts_with("0") {
            String::from("0")
        } else {
            nums.join("")
        }
    }
}

fn main() {
    let test_cases = [vec![10, 2], vec![3, 30, 34, 5, 9], vec![9, 91, 912]];

    for nums in test_cases {
        println!("{}", Solution::largest_number(nums))
    }
}
