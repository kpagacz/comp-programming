// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/description/
struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut bit_count = [0i32; 32usize];
        let mut k_bits = [false; 32];

        k_bits.iter_mut().enumerate().for_each(|(i, val)| {
            if (k >> i) & 1 == 1 {
                *val = true;
            }
        });

        for num in nums {
            bit_count.iter_mut().enumerate().for_each(|(i, count)| {
                if (num >> i) & 1 == 1 {
                    *count += 1;
                }
            });
        }

        bit_count
            .iter()
            .zip(k_bits)
            .filter(|(&count, is_one)| (count % 2 == 1) != (*is_one))
            .count() as _
    }

    fn smart_way(num: Vec<i32>, k: i32) -> i32 {
        num.into_iter().fold(k, |acc, num| acc ^ num).count_ones() as _
    }
}

fn main() {
    let test_cases = [(vec![2, 1, 3, 4], 1), (vec![2, 0, 2, 0], 0)];
    for (nums, k) in test_cases {
        println!("{}", Solution::min_operations(nums, k));
    }
}
