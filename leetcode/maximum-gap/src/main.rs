// https://leetcode.com/problems/maximum-gap/
pub struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let sorted = Solution::radix_sort(nums);
        sorted
            .windows(2)
            .map(|window| window[1] - window[0])
            .max()
            .unwrap()
    }

    // This is super clever because who said that we have to do the radix
    // sort using 10 as radix? Why not use 256 which is faster to calculate
    pub fn maximum_gap2(mut nums: Vec<i32>) -> i32 {
        fn count_sort<F: Fn(i32) -> i32>(
            nums: &mut Vec<i32>,
            by_fn: F,
            counter: &mut Vec<Vec<i32>>,
        ) {
            nums.iter().for_each(|num| {
                let key = by_fn(*num) as usize;
                counter[key].push(*num);
            });
            nums.clear();
            for vec in counter {
                nums.append(vec);
            }
        }
        let mut counter = vec![Vec::with_capacity(nums.len()); 256];
        count_sort(&mut nums, |num| num & 0xff, &mut counter);
        count_sort(&mut nums, |num| (num >> 8) & 0xff, &mut counter);
        count_sort(&mut nums, |num| (num >> 16) & 0xff, &mut counter);
        count_sort(&mut nums, |num| (num >> 24) & 0xff, &mut counter);

        nums.windows(2)
            .map(|window| window[1] - window[0])
            .max()
            .unwrap_or(0)
    }

    fn radix_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let max = *nums.iter().max().unwrap();
        let mut exp = 1;

        while max / exp > 0 {
            nums = Solution::count_sort(nums, exp);
            exp *= 10;
        }

        nums
    }

    fn count_sort(nums: Vec<i32>, exp: i32) -> Vec<i32> {
        let mut sorted = vec![0; nums.len()];
        let mut count = [0; 10];

        for num in &nums {
            count[((*num / exp) % 10) as usize] += 1;
        }

        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        for num in nums.into_iter().rev() {
            sorted[count[((num / exp) % 10) as usize] - 1] = num;
            count[((num / exp) % 10) as usize] -= 1;
        }

        sorted
    }
}

fn main() {
    println!("Hello, world!");
}
