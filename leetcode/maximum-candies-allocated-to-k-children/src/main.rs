// https://leetcode.com/problems/maximum-candies-allocated-to-k-children/description/?envType=daily-question&envId=2025-03-14
pub struct Solution;

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let max = *candies.iter().max().unwrap();
        let total = candies.iter().map(|candies| *candies as i64).sum::<i64>();
        let candidates: Vec<_> = (1..=max).collect();

        fn can_divide(candies: &[i32], in_pile: i32, k: i64) -> bool {
            candies
                .iter()
                .map(|&count| count / in_pile)
                .map(|piles| piles as i64)
                .sum::<i64>()
                >= k
        }

        let pp = candidates.partition_point(|candidate_in_pile| {
            *candidate_in_pile as i64 * k <= total && can_divide(&candies, *candidate_in_pile, k)
        });

        if pp == candidates.len() {
            pp as _
        } else {
            candidates[pp] - 1
        }
    }
}

fn main() {
    let test_cases = [(vec![5, 8, 6], 3), (vec![2, 5], 11)];
    for (candies, k) in test_cases {
        println!("{}", Solution::maximum_candies(candies, k));
    }
}
