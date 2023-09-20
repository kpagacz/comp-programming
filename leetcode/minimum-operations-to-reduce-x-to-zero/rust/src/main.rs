// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/
pub struct Solution {}

impl Solution {
    // must be doable in O(n), so here it is
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let nums_sum: i32 = nums.iter().sum();
        let to_collect = nums_sum - x;
        let mut max_elems = i32::MIN;
        let mut current_sum = 0;

        let (mut left, mut right) = (0, 0);

        println!("{to_collect}");
        while right < nums.len() {
            current_sum += nums[right];

            while left <= right && current_sum > to_collect {
                current_sum -= nums[left];
                left += 1;
            }

            if current_sum == to_collect {
                max_elems = max_elems.max((right - left) as i32);
            }

            right += 1;
        }

        if max_elems == i32::MIN {
            -1
        } else {
            nums.len() as i32 - max_elems - 1
        }
    }

    // SLOW 20% runtime
    pub fn min_operations_slow(nums: Vec<i32>, x: i32) -> i32 {
        use std::collections::VecDeque;
        let mut prefix_sums = nums
            .iter()
            .scan(0, |sum, el| {
                *sum += *el;
                Some(*sum)
            })
            .collect::<VecDeque<_>>();
        prefix_sums.push_front(0);

        let mut minimum_taken = usize::MAX;

        for taken_from_left in 0..=nums.len() {
            let current_sum = prefix_sums[taken_from_left];

            if taken_from_left > minimum_taken {
                break;
            }
            let mut left = taken_from_left;
            let mut right = nums.len();

            while left < right {
                let mid = (left + right) / 2;

                match x.cmp(&(current_sum + prefix_sums.back().unwrap() - prefix_sums[mid])) {
                    std::cmp::Ordering::Less => {
                        left = mid + 1;
                    }
                    std::cmp::Ordering::Equal => {
                        left = mid;
                        break;
                    }
                    std::cmp::Ordering::Greater => {
                        right = mid;
                    }
                }
            }

            if x == current_sum + prefix_sums.back().unwrap() - prefix_sums[left] {
                if taken_from_left + nums.len() - left < minimum_taken {
                    minimum_taken = taken_from_left + nums.len() - left;
                }
            }
        }

        minimum_taken as i32
    }
}

fn main() {
    let test_case = vec![3, 2, 20, 1, 1, 3];
    println!("{}", Solution::min_operations(test_case, 10));
}
