// https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/description/
pub struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let lists_n = nums.len();
        let nums_total: usize = nums.iter().map(|list| list.len()).sum();
        let mut its: Vec<_> = nums
            .into_iter()
            .enumerate()
            .map(|(id, list)| list.into_iter().zip(std::iter::repeat(id)))
            .collect();
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut sorting_heap = BinaryHeap::default();
        for it in its.iter_mut() {
            if let Some(el) = it.next() {
                sorting_heap.push(Reverse(el));
            }
        }

        let mut all_nums = Vec::with_capacity(nums_total);
        let mut visited_lists = vec![0; lists_n]; // nums from the ith list
        let mut visited_lists_count = 0;
        let mut lowest_num = 0;
        let mut smallest_range = vec![0, i32::MAX];
        fn range_span(range: &[i32]) -> i32 {
            range[1] - range[0]
        }

        while let Some(Reverse((num, list_id))) = sorting_heap.pop() {
            visited_lists[list_id] += 1;
            if visited_lists[list_id] == 1 {
                visited_lists_count += 1;
            }
            all_nums.push((num, list_id));

            while visited_lists_count == lists_n {
                let new_range = vec![all_nums[lowest_num].0, num];
                if range_span(&new_range) < range_span(&smallest_range) {
                    smallest_range = new_range;
                }

                let list_id_of_removed_el = all_nums[lowest_num].1;
                visited_lists[list_id_of_removed_el] -= 1;
                if visited_lists[list_id_of_removed_el] == 0 {
                    visited_lists_count -= 1;
                }
                lowest_num += 1;
            }

            if let Some(el) = its[list_id].next() {
                sorting_heap.push(Reverse(el));
            }
        }

        smallest_range
    }
}

fn main() {
    println!("Hello, world!");
}
