// https://leetcode.com/problems/max-dot-product-of-two-subsequences/description/
pub struct Solution {}

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![i32::MIN; nums2.len() + 1]; nums1.len() + 1];

        (1..=nums1.len()).for_each(|x| {
            (1..=nums2.len()).for_each(|y| {
                dp[x][y] = (dp[x - 1][y - 1].max(0) + nums1[x - 1] * nums2[y - 1])
                    .max(dp[x - 1][y])
                    .max(dp[x][y - 1])
            })
        });

        dp[nums1.len()][nums2.len()]
    }
}

fn main() {
    let test_cases = vec![
        // (vec![1, 20], vec![10, 1]),
        // (vec![1, 10, 1], vec![20, 1, 1, 4]),
        // (vec![1, 10, 1, 1, 1, 1, 70], vec![20, 1, 1, 4, 1, 1]),
        // (
        //     vec![1, 10, 1, 1, 1, 1, 70, 1, 1, 1, 1, 3],
        //     vec![20, 1, 1, 4, 1, 1],
        // ),
        // (vec![-7], vec![1, 1, -10]),
        (vec![-5, 3, -5, -3, 1], vec![-2, 4, 2, 5, -5]),
    ];
    for (nums1, nums2) in test_cases {
        println!("{}", Solution::max_dot_product(nums1, nums2));
    }
}
