// https://leetcode.com/problems/minimum-equal-sum-of-two-arrays-after-replacing-zeros/description/?envType=daily-question&envId=2025-05-10
pub struct Solution;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let zeros2 = nums2.iter().filter(|&&x| x == 0).count();
        let sum2 = nums2.iter().map(|num| *num as i64).sum::<i64>() + zeros2 as i64;
        let zeros1 = nums1.iter().filter(|&&x| x == 0).count();
        let sum1 = nums1.iter().map(|num| *num as i64).sum::<i64>() + zeros1 as i64;
        match sum1.cmp(&sum2) {
            std::cmp::Ordering::Less => {
                if zeros1 > 0 {
                    sum2
                } else {
                    -1
                }
            }
            std::cmp::Ordering::Equal => sum1,
            std::cmp::Ordering::Greater => {
                if zeros2 > 0 {
                    sum1
                } else {
                    -1
                }
            }
        }
    }
}

fn main() {
    let test_cases = [(vec![3, 2, 0, 1, 0], vec![6, 5, 0])];
    for (nums1, nums2) in test_cases {
        let result = Solution::min_sum(nums1, nums2);
        println!("Result: {}", result);
    }
}
