// https://leetcode.com/problems/find-unique-binary-string/description/
pub struct Solution {}

impl Solution {
    pub fn cantor(nums: Vec<String>) -> String {
        let mut answer = String::from("");
        for i in 0..nums.len() {
            if &nums[i][i..(i + 1)] == "1" {
                answer += "0";
            } else {
                answer += "1";
            }
        }
        answer
    }
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let nums: Vec<_> = nums
            .iter()
            .map(|num| usize::from_str_radix(num, 2).unwrap())
            .collect();
        for i in 0..=n {
            if !nums.contains(&i) {
                return format!("{i:0width$b}", width = n);
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
