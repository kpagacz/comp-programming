// https://leetcode.com/problems/arithmetic-slices-ii-subsequence/description/
pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut sorted_suffixes = vec![];
        for i in 0..nums.len() {
            let mut suffix = (i + 1..nums.len()).collect::<Vec<_>>();
            suffix.sort_by_key(|index| nums[*index]);
            sorted_suffixes.push(suffix);
        }
        // for suffix in &sorted_suffixes {
        //     println!("{suffix:?}");
        // }

        use std::collections::HashMap;
        fn arithmetic_slices_rec(
            first: usize,
            second: usize,
            cache: &mut HashMap<(usize, usize), i32>,
            nums: &[i32],
            sorted_suffixes: &[Vec<usize>],
        ) -> i32 {
            if let Some(&cached) = cache.get(&(first, second)) {
                return cached;
            }
            let to_find = 2 * nums[second] as i64 - nums[first] as i64;
            // println!("Sorted suffix: {:?}", &sorted_suffixes[second]);
            // println!("Sorted suffix len: {}", sorted_suffixes[second].len());
            let mut first_next_second =
                sorted_suffixes[second].partition_point(|&key| (nums[key] as i64) < to_find);
            let mut next_seconds = vec![];
            while first_next_second < sorted_suffixes[second].len()
                && nums[sorted_suffixes[second][first_next_second]] as i64 == to_find
            {
                next_seconds.push(first_next_second);
                first_next_second += 1;
            }
            let ans = if !next_seconds.is_empty() {
                // println!(
                //     "For {first} {second} found the next elements: {:?}",
                //     &sorted_suffixes[second]
                //         [*next_seconds.first().unwrap()..=*next_seconds.last().unwrap()],
                // );
                next_seconds
                    .into_iter()
                    .map(|first_next_second| {
                        1 + arithmetic_slices_rec(
                            second,
                            sorted_suffixes[second][first_next_second],
                            cache,
                            nums,
                            sorted_suffixes,
                        )
                    })
                    .sum::<i32>()
            } else {
                0
            };
            cache.insert((first, second), ans);
            // println!("For {first} {second} returning {ans}");
            ans
        }
        let mut cache = HashMap::new();
        let mut answer = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                answer += arithmetic_slices_rec(i, j, &mut cache, &nums, &sorted_suffixes)
            }
        }
        answer
    }
}

fn main() {
    let test_cases = [
        vec![2, 4, 6, 8, 10],
        vec![7, 7, 7, 7, 7],
        vec![3, 1, -1, 1, 3, 1],
    ];
    for nums in test_cases {
        println!("Array: {nums:?}");
        println!("{}", Solution::number_of_arithmetic_slices(nums));
    }
}
