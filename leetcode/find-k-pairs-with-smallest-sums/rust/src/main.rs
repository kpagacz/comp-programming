// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/

pub struct Solution {}
impl Solution {
    // Runtime42 ms Beats 83.33%
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        let mut pqueue = std::collections::BinaryHeap::from(
            nums1
                .iter()
                .enumerate()
                .map(|(i, &el)| (-nums2[0] - el, i, 0usize))
                .collect::<Vec<(i32, usize, usize)>>(),
        );
        let mut pairs_needed = std::cmp::min(k as i64, nums1.len() as i64 * nums2.len() as i64);
        while pairs_needed > 0 {
            let (_, first, second) = pqueue.pop().unwrap();
            answer.push(vec![nums1[first], nums2[second]]);
            if second + 1 < nums2.len() {
                pqueue.push((-nums1[first] - nums2[second + 1], first, second + 1));
            };
            pairs_needed -= 1;
        }
        answer
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 7, 11], vec![2, 4, 6], 3),
        (vec![1, 1, 2], vec![1, 2, 3], 2),
        (vec![1, 2], vec![3], 3),
    ];

    for (nums1, nums2, k) in test_cases {
        println!("{:?}", Solution::k_smallest_pairs(nums1, nums2, k));
    }
}
