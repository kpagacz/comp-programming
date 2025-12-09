// https://leetcode.com/problems/count-special-triplets/description/?envType=daily-question&envId=2025-12-09
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut seen = vec![0usize; 100_001];
        let mut seen_pairs = vec![0usize; 100_001];
        let mut triplets = 0;
        const MOD: usize = 10usize.pow(9) + 7;
        for num in nums {
            let num = num as usize;
            if num % 2 == 0 {
                triplets += seen_pairs[num / 2];
                triplets %= MOD;
            }
            if num * 2 < seen.len() {
                seen_pairs[num] += seen[num * 2];
                seen_pairs[num] %= MOD;
            }
            seen[num] += 1;
            seen[num] %= MOD;
        }
        triplets as _
    }
}

fn main() {
    let test_cases = [
        (vec![6, 3, 6], 1),
        (vec![0, 1, 0, 0], 1),
        (vec![8, 4, 2, 8, 4], 2),
        (vec![6, 3, 6, 3, 6, 3, 6, 3, 3, 3, 6, 6, 6], 64),
    ];
    for (nums, exp) in test_cases {
        println!("{}, exp: {exp}", Solution::special_triplets(nums));
    }
}
