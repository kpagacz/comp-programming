// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-ii/description/
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let k = k as usize;
        let dist = dist as usize;
        let first = nums[0] as i64;
        use std::collections::BTreeMap;
        let mut sum_elements = BTreeMap::new();
        let mut rest = BTreeMap::new();
        nums.iter().rev().take(dist + 1).for_each(|num| {
            sum_elements
                .entry(*num as i64)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });

        let custom_pop_last = |map: &mut BTreeMap<i64, u32>| -> i64 {
            let last_key = *map.last_key_value().unwrap().0;
            let e = map.entry(last_key).or_insert(0);
            *e -= 1;
            if *e == 0 {
                map.remove(&last_key);
            }
            last_key
        };
        let custom_pop_front = |map: &mut BTreeMap<i64, u32>| -> i64 {
            let first_key = *map.first_key_value().unwrap().0;
            let e = map.entry(first_key).or_default();
            *e -= 1;
            if *e == 0 {
                map.remove(&first_key);
            }
            first_key
        };

        for _ in 0..dist + 2 - k {
            rest.entry(custom_pop_last(&mut sum_elements))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        let mut sum = sum_elements
            .iter()
            .map(|(key, value)| *key * *value as i64)
            .sum::<i64>();
        let mut min_sum = sum;

        for i in (1..nums.len() - dist - 1).rev() {
            // Removed the element that goes out of dist
            let removed = nums[i + dist + 1] as i64;
            let mut needs_one_from_rest = false;
            let removed_el_map = if removed
                < rest
                    .first_key_value()
                    .map(|(key, _)| *key)
                    .unwrap_or(i64::MAX)
            {
                sum -= removed;
                needs_one_from_rest = true;
                &mut sum_elements
            } else {
                &mut rest
            };
            let e = removed_el_map.entry(removed).or_default();
            *e -= 1;
            if *e == 0 {
                removed_el_map.remove(&removed);
            }
            // Add element
            let num = nums[i] as i64;
            if num <= *sum_elements.last_key_value().unwrap().0 {
                sum_elements.entry(num).and_modify(|e| *e += 1).or_insert(1);
                sum += num;
                if !needs_one_from_rest {
                    let last = custom_pop_last(&mut sum_elements);
                    sum -= last;
                    rest.entry(last).and_modify(|e| *e += 1).or_insert(1);
                }
            } else {
                rest.entry(num).and_modify(|e| *e += 1).or_insert(1);
                if needs_one_from_rest {
                    let first = custom_pop_front(&mut rest);
                    sum_elements
                        .entry(first)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    sum += first;
                }
            }

            // Set min sum
            min_sum = min_sum.min(sum);
        }

        first + min_sum
    }
}

fn main() {
    let test_cases = [
        (vec![1, 3, 2, 6, 4, 2], 3, 3, 5),
        (vec![10, 1, 2, 2, 2, 1], 4, 3, 15),
        (vec![10, 8, 18, 9], 3, 1, 36),
    ];

    for (nums, k, dist, exp) in test_cases {
        println!("nums: {nums:?}");
        println!("{} exp {exp}", Solution::minimum_cost(nums, k, dist));
    }
}
