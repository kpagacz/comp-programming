// https://leetcode.com/problems/permutations/description/
pub struct Solution {}

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Self::permute1(&mut nums)
        Self::permute2(&mut nums)
    }

    // Runtime0 ms Beats 100%
    // Memory2.1 MB Beats 93.44%
    fn permute2(nums: &mut [i32]) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        Self::heap_algorithm(nums.len(), nums, &mut answer);
        answer
    }
    fn heap_algorithm(k: usize, nums: &mut [i32], mem: &mut Vec<Vec<i32>>) {
        if k == 1 {
            mem.push(nums.iter().map(i32::clone).collect::<Vec<_>>());
            return;
        }

        Self::heap_algorithm(k - 1, nums, mem);
        for i in 0..(k - 1) {
            if k % 2 == 0 {
                nums.swap(i, k - 1);
            } else {
                nums.swap(0, k - 1);
            }
            Self::heap_algorithm(k - 1, nums, mem);
        }
    }
    // Runtime1 ms Beats 87.70%
    // Memory2.2 MB Beats 59.2%
    fn permute1(nums: &mut [i32]) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut answer = vec![nums.iter().map(i32::clone).collect::<Vec<i32>>()];
        while Self::next_permutation(nums) {
            answer.push(nums.iter().map(i32::clone).collect::<Vec<_>>());
        }
        answer
    }
    fn next_permutation(nums: &mut [i32]) -> bool {
        use std::cmp::Ordering;
        let last_rev_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
            Some(index) => index,
            None => {
                nums.reverse();
                return false;
            }
        };
        let swap_with = nums[last_rev_ascending + 1..]
            .binary_search_by(|el| i32::cmp(&nums[last_rev_ascending], el).then(Ordering::Less))
            .unwrap_err();
        nums.swap(last_rev_ascending, last_rev_ascending + swap_with);
        nums[last_rev_ascending + 1..].reverse();
        true
    }
}
fn main() {
    let test_cases = vec![
        vec![5, 4, 3, 2, 1],
        // vec![4,3,2,1]
    ];

    for test in test_cases {
        println!("{:?}", Solution::permute(test));
    }
}
