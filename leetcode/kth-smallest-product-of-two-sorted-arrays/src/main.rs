// https://leetcode.com/problems/kth-smallest-product-of-two-sorted-arrays/description/?envType=daily-question&envId=2025-06-25
pub struct Solution;

impl Solution {
    // Returns the correct answer it's just too slow for large inputs.
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        fn products_lower_equal_than(nums2: &[i32], num1: i64, target: i64) -> usize {
            if num1 >= 0 {
                nums2.partition_point(|&num2| num2 as i64 * num1 <= target)
            } else {
                let higher = nums2.partition_point(|&num2| num2 as i64 * num1 > target);
                nums2.len() - higher
            }
        }

        let mut left = -10i64.pow(10);
        let mut right = 10i64.pow(10);
        let k = k as usize;
        let nums1: Vec<_> = nums1.into_iter().map(|num| num as i64).collect();
        while left < right {
            let middle = left + (right - left) / 2;

            let mut count = 0;
            for &num1 in &nums1 {
                count += products_lower_equal_than(&nums2, num1, middle);
            }
            if count < k {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        // let mut count = 0;
        // for &num1 in &nums1 {
        //     count += products_lower_equal_than(&nums2, num1, left);
        // }
        // assert_eq!(count, k);
        // If we counted prodcuts from 0 to k-th than we would
        // look for lower than products, not lower or equal.
        left
    }
}

fn main() {
    let test_cases = [
        (vec![2, 5], vec![3, 4], 2, 8),
        (vec![-4, -2, 0, 3], vec![2, 4], 6, 0),
        (vec![-2, -1, 0, 1, 2], vec![-3, -1, 2, 4, 5], 3, -6),
    ];
    for (nums1, nums2, k, exp) in test_cases {
        println!(
            "{} exp: {exp}",
            Solution::kth_smallest_product(nums1, nums2, k)
        );
    }
}
