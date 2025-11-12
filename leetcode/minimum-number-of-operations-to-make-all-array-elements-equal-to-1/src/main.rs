// https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/description/?envType=daily-question&envId=2025-11-12
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut shortest = usize::MAX;

        if nums.iter().all(|num| *num == 1) {
            return 0;
        }

        let ones = nums.iter().filter(|&&num| num == 1).count();
        if ones != 0 {
            return (nums.len() - ones) as _;
        }

        for start in 0..nums.len() {
            for end in start + 1..nums.len() {
                let mut current_gcd = nums[start];
                for &num in &nums[start..=end] {
                    current_gcd = Solution::gcd(current_gcd, num);
                }
                if current_gcd == 1 {
                    shortest = shortest.min(end - start);
                }
            }
        }

        if shortest == usize::MAX {
            -1
        } else {
            (shortest + nums.len() - 1) as _
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Solution::gcd(b, a % b) }
    }
}

fn main() {
    let test_cases = [
        (vec![2, 6, 3, 4]),
        (vec![2, 10, 6, 14]),
        (vec![6, 10, 15]),
        (vec![1, 1]),
        (vec![1, 2]),
    ];

    for nums in test_cases {
        println!("{}", Solution::min_operations(nums));
    }
}
