// https://leetcode.com/problems/the-number-of-beautiful-subsets/description/
pub struct Solution;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        fn rec(nums: &[i32], nums_so_far: &mut [i32], k: i32, res: &mut i32) {
            if nums.is_empty() {
                *res += 1;
                return;
            }

            let num = nums[0];
            let (below, above) = (num - k, num + k);

            if (below < 0 || nums_so_far[below as usize] == 0)
                && ((above > 1000) || (nums_so_far[above as usize] == 0))
            {
                nums_so_far[num as usize] += 1;
                rec(&nums[1..], nums_so_far, k, res);
                nums_so_far[num as usize] -= 1;
            }
            rec(&nums[1..], nums_so_far, k, res);
        }

        let mut nums_so_far = [0; 1001];
        let mut res = 0;
        rec(&nums, &mut nums_so_far, k, &mut res);
        res - 1
    }
}

fn main() {
    let test_cases = [(vec![2, 4, 6], 2)];
    for (nums, k) in test_cases {
        println!("{}", Solution::beautiful_subsets(nums, k));
    }
}
