// https://leetcode.com/problems/frequency-of-the-most-frequent-element/description/
pub struct Solution {}

impl Solution {
    pub fn max_frequency_smart(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut max = usize::MIN;
        let mut previous = 0;
        let mut start = 0;
        let mut required = 0;
        for (i, &num) in nums.iter().enumerate() {
            required += (num - previous) * (i - start) as i32;

            while required > k {
                required -= num - nums[start];
                start += 1;
            }

            max = max.max(i - start + 1);
            previous = num;
        }
        max as i32
    }
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let max = *nums.iter().max().unwrap() as usize;
        let min = *nums.iter().min().unwrap() as usize;
        let mut counts = vec![0; 10usize.pow(5) + 1];
        for num in nums {
            counts[num as usize] += 1;
        }

        fn next_left(left: usize, counts: &[i32], min: usize) -> usize {
            let mut ret = left - 1;
            while ret > min - 1 && counts[ret] == 0 {
                ret -= 1;
            }
            ret
        }

        let mut max_count = counts[max];
        let mut current_cost = 0;
        let mut current_count = counts[max];
        let mut left = max;
        let mut i = max;
        while i != min - 1 {
            loop {
                let next = next_left(left, &counts, min);
                if next == min - 1 {
                    max_count = max_count.max(current_count);
                    break;
                }
                let next_cost = (i - next) as i32 * counts[next];

                if current_cost + next_cost <= k {
                    current_cost += next_cost;
                    left = next;
                    current_count += counts[left];
                    max_count = max_count.max(current_count);
                } else {
                    let partial = (k - current_cost) / (i - next) as i32;
                    max_count = max_count.max(current_count + partial);
                    break;
                }
            }

            let next_i = next_left(i, &counts, min);
            current_cost -= (current_count - counts[i]) * (i - next_i) as i32;
            current_count -= counts[i];
            i = next_i;
        }
        max_count
    }
    pub fn max_frequency_tree(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BTreeMap;
        let mut freq = BTreeMap::new();
        for num in nums {
            *freq.entry(num).or_insert(0) += 1;
        }
        // println!("freq: {freq:?}");

        let mut max_count = i32::MIN;
        let mut current_cost = 0;
        let mut curr_num = *freq.iter().rev().next().unwrap().0;
        let mut curr_count = *freq.iter().rev().next().unwrap().1;
        for (&num, &count) in freq.iter().rev() {
            // println!("max_count: {max_count} curr_cost:{current_cost} curr_num: {curr_num} curr_count: {curr_count}");
            while let Some(next) = freq.range(..curr_num).next_back() {
                let (&next_num, &next_count) = next;
                // println!("next: {next_num} {next_count}");
                if (num - next_num) * next_count + current_cost <= k {
                    // println!("still under cost");
                    current_cost += (num - next_num) * next_count;
                    curr_count += next_count;
                    curr_num = next_num;
                } else {
                    // println!("cost exceeded");
                    // add at least some of the nums to the curr_count
                    let partial = (k - current_cost) / (num - next_num);
                    max_count = max_count.max(curr_count + partial);

                    let (&next_num, _) = freq.range(..num).next_back().unwrap();
                    current_cost -= (curr_count - count) * (num - next_num);
                    curr_count -= count;
                    break;
                }
            }
            max_count = max_count.max(curr_count);
        }
        max_count
    }
}
fn main() {
    let test_cases = vec![
        (vec![1, 2, 4], 5),
        (vec![1, 4, 8, 13, 14, 20], 8),
        (vec![3, 9, 6], 2),
        (vec![1, 9, 6, 4], 2),
        (vec![1, 1, 1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3], 8),
        (
            vec![
                9979, 9938, 9947, 9916, 9995, 9981, 9981, 9931, 9984, 9942, 9946, 9946, 9945, 9931,
                9908, 9920, 9929, 9917, 9904, 9945, 9963, 9910, 9942, 9965, 9915, 9981, 9908, 9919,
                9975, 9904, 9934, 9922, 9989, 9946, 9928, 9928, 9940, 9941, 9995, 9905, 9903, 9980,
                9917, 9940, 9910, 9994, 9909, 9965, 9972, 9931, 9975, 9913, 9983, 9943, 9996, 9917,
                9994, 9991, 9948, 9961, 9921, 9981, 9928, 9933, 9905, 9957, 9953, 9940, 9958, 9982,
                9900, 9912, 9959, 9992, 9978, 9988, 9940, 9985, 9945, 9900, 9956, 9976, 9972, 9914,
                9903, 9978, 9965, 9987, 9926, 9963, 9968, 9962, 9995, 9963, 9960, 9986, 9916, 9911,
                9976, 9988, 9952, 9914, 9934, 9929, 9962, 9999, 9988, 9901, 9925, 9983, 9991, 9915,
                9930, 9949, 9931, 9944, 9947, 9921, 9982, 9984, 9998, 9945, 9907, 9900, 9992, 9945,
                9995, 9941, 9930, 9918, 9961, 9960, 9900, 9952, 9952, 9954, 9976, 9970, 9990, 9947,
                9910, 9908, 9935, 9971, 9971, 10000, 9941, 9983, 9949, 9985, 9992, 9996, 9918,
                9930, 9994, 9970, 9989, 9975, 9960, 9973, 9993, 9900, 9915, 9974, 9966, 9978, 9926,
                9937, 9936, 9952, 9996, 9996, 9912, 9915, 9976, 9976, 9901, 9926, 9959, 9989, 9976,
                9904, 9999, 9925, 9934, 9947, 9950, 9985, 9985, 9932, 9922, 9962, 9962, 9993, 9912,
                9924, 9992, 9941, 9959, 9954, 9943, 9995, 9992, 9928, 9992, 9920, 9984, 9917, 9976,
                9971, 9927, 9917, 9923, 9948, 9929, 9990, 9990, 9921, 9989, 9910, 9921,
            ],
            2636,
        ),
    ];
    for (nums, k) in test_cases {
        println!("{}", Solution::max_frequency(nums, k));
    }
}
