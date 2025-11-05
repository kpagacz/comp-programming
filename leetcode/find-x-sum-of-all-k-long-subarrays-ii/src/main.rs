// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-ii/description/?envType=daily-question&envId=2025-11-05
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        use std::collections::BTreeSet;
        use std::collections::HashMap;
        let k = k as usize;
        let x = x as usize;
        let mut top_freqs: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut bottom_freqs: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut freqs = HashMap::new();
        let mut top_sum = 0i64;

        fn balance_sets(
            top_freqs: &mut BTreeSet<(i32, i32)>,
            bottom_freqs: &mut BTreeSet<(i32, i32)>,
            top_sum: &mut i64,
            limit: usize,
        ) {
            if let (Some((freq, num)), Some((bottom_freq, bottom_num))) =
                (top_freqs.first(), bottom_freqs.last())
                && freq.cmp(bottom_freq).then(num.cmp(bottom_num)) == std::cmp::Ordering::Less
            {
                let smallest_top = top_freqs.pop_first().unwrap();
                *top_sum -= smallest_top.0 as i64 * smallest_top.1 as i64;
                bottom_freqs.insert(smallest_top);
                let largest_bottom = bottom_freqs.pop_last().unwrap();
                *top_sum += largest_bottom.0 as i64 * largest_bottom.1 as i64;
                top_freqs.insert(largest_bottom);
            };
            if top_freqs.len() > limit {
                let smallest_top = top_freqs.pop_first().unwrap();
                *top_sum -= smallest_top.0 as i64 * smallest_top.1 as i64;
                bottom_freqs.insert(smallest_top);
            }
        }

        fn remove_num(
            num: i32,
            top_freqs: &mut BTreeSet<(i32, i32)>,
            bottom_freqs: &mut BTreeSet<(i32, i32)>,
            freqs: &mut HashMap<i32, i32>,
            top_sum: &mut i64,
            limit: usize,
        ) {
            let freq = *freqs.get(&num).unwrap_or(&0);
            assert!(freq != 0);
            freqs.insert(num, freq - 1);
            match (
                top_freqs.remove(&(freq, num)),
                bottom_freqs.remove(&(freq, num)),
            ) {
                (true, true) => unreachable!("Num cannot be in both sets at the same time"),
                (true, false) => {
                    top_freqs.insert((freq - 1, num));
                    *top_sum -= num as i64;
                }
                (false, true) => {
                    bottom_freqs.insert((freq - 1, num));
                }
                (false, false) => unreachable!("Num must be in at least one set"),
            };

            balance_sets(top_freqs, bottom_freqs, top_sum, limit);
        }

        fn add_num(
            num: i32,
            top_freqs: &mut BTreeSet<(i32, i32)>,
            bottom_freqs: &mut BTreeSet<(i32, i32)>,
            freqs: &mut HashMap<i32, i32>,
            top_sum: &mut i64,
            limit: usize,
        ) {
            let freq = *freqs.get(&num).unwrap_or(&0);
            freqs.insert(num, freq + 1);
            if freq != 0 {
                match (
                    top_freqs.remove(&(freq, num)),
                    bottom_freqs.remove(&(freq, num)),
                ) {
                    (true, true) => unreachable!("Num cannot be in both sets at the same time"),
                    (true, false) => {
                        top_freqs.insert((freq + 1, num));
                        *top_sum += num as i64;
                    }
                    (false, true) => {
                        bottom_freqs.insert((freq + 1, num));
                    }
                    (false, false) => unreachable!("Num must be in at least one set"),
                };
            } else {
                top_freqs.insert((1, num));
                *top_sum += num as i64;
            }

            balance_sets(top_freqs, bottom_freqs, top_sum, limit);
        }

        for &num in nums.iter().take(k) {
            add_num(
                num,
                &mut top_freqs,
                &mut bottom_freqs,
                &mut freqs,
                &mut top_sum,
                x,
            );
        }
        let mut answer = Vec::with_capacity(nums.len() - k + 1);
        answer.push(top_sum);
        for i in k..nums.len() {
            let num = nums[i];
            let removed_num = nums[i - k];
            remove_num(
                removed_num,
                &mut top_freqs,
                &mut bottom_freqs,
                &mut freqs,
                &mut top_sum,
                x,
            );
            add_num(
                num,
                &mut top_freqs,
                &mut bottom_freqs,
                &mut freqs,
                &mut top_sum,
                x,
            );
            answer.push(top_sum);
            // println!(
            //     "top_freqs: {top_freqs:?}\nbottom_freqs{bottom_freqs:?}\nfreqs: {freqs:?}\nsum:{top_sum}"
            // );
        }
        answer
    }
}

fn main() {
    let test_cases = [(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2)];
    for (nums, k, x) in test_cases {
        println!("{:?}", Solution::find_x_sum(nums, k, x));
    }
}
