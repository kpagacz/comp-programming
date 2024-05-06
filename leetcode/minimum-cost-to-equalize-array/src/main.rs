// https://leetcode.com/problems/minimum-cost-to-equalize-array/description/
pub struct Solution;

impl Solution {
    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        let nums: Vec<_> = nums.into_iter().map(|num| num as usize).collect();
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let sum = nums.iter().sum::<usize>();

        let mut answer = usize::MAX;

        let (cost1, cost2) = (cost1 as usize, cost2 as usize);
        for up_to in (max..=2 * max).rev() {
            let mut differences = up_to * nums.len() - sum;
            let mut cost = 0;

            if (up_to - min) * 2 > differences {
                let rest = 2 * (up_to - min) - differences;
                cost += cost1 * rest;
                differences -= rest;
            }

            if differences % 2 == 1 {
                differences -= 1;
                cost += cost1;
            }

            cost += usize::min((differences / 2) * cost2, differences * cost1);
            answer = answer.min(cost);
        }

        (answer % (10usize.pow(9) + 7)) as _
    }
}

fn main() {
    let test_cases = [(vec![4, 3], 2, 6), (vec![4, 1], 5, 2)];

    for (nums, cost1, cost2) in test_cases {
        println!("vec: {nums:?}");
        println!(
            "{}",
            Solution::min_cost_to_equalize_array(nums, cost1, cost2)
        );
    }
}
