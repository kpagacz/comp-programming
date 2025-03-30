// https://leetcode.com/problems/create-maximum-number/
pub struct Solution;

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        fn find_max_num(digits: &[i32], size: usize) -> Vec<i32> {
            let mut stack = Vec::with_capacity(size);
            let mut allowed_to_drop = digits.len() - size;
            for &digit in digits {
                while allowed_to_drop > 0 && !stack.is_empty() && digit > *stack.last().unwrap() {
                    allowed_to_drop -= 1;
                    stack.pop();
                }
                stack.push(digit);
            }
            stack.truncate(size);
            stack
        }

        fn merge(first_num: Vec<i32>, second_num: Vec<i32>) -> Vec<i32> {
            let mut merged = Vec::with_capacity(first_num.len() + second_num.len());

            let mut first_it = 0;
            let mut second_it = 0;
            while first_it < first_num.len() || second_it < second_num.len() {
                match first_num[first_it..].cmp(&second_num[second_it..]) {
                    std::cmp::Ordering::Greater => {
                        merged.push(first_num[first_it]);
                        first_it += 1;
                    }
                    _ => {
                        merged.push(second_num[second_it]);
                        second_it += 1;
                    }
                }
            }
            merged
        }

        let k = k as usize;
        (0usize..=k)
            .filter(|&i| i <= nums1.len() && k - i <= nums2.len())
            .map(|i| merge(find_max_num(&nums1, i), find_max_num(&nums2, k - i)))
            .max()
            .unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
