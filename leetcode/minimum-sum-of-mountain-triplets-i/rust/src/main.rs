// https://leetcode.com/problems/minimum-sum-of-mountain-triplets-i/
// https://leetcode.com/problems/minimum-sum-of-mountain-triplets-ii/
pub struct Solution {}
impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let prefix_min = nums
            .iter()
            .scan(nums[0], |acc, &num| {
                *acc = (*acc).min(num);
                Some(*acc)
            })
            .collect::<Vec<i32>>();
        let suffix_min = nums
            .iter()
            .rev()
            .scan(nums[nums.len() - 1], |acc, &num| {
                *acc = (*acc).min(num);
                Some(*acc)
            })
            .collect::<Vec<i32>>();
        // println!("{prefix_min:?} {suffix_min:?}");
        if nums.len() < 3 {
            return -1;
        }
        let min = nums
            .iter()
            .enumerate()
            .skip(1)
            .take(nums.len() - 2)
            .map(|(id, &num)| {
                // println!(
                //     "{} {}",
                //     prefix_min[id - 1],
                //     suffix_min[nums.len() - 1 - (id + 1)]
                // );
                if num > prefix_min[id - 1] && num > suffix_min[nums.len() - 1 - (id + 1)] {
                    num + prefix_min[id - 1] + suffix_min[nums.len() - 1 - (id + 1)]
                } else {
                    i32::MAX
                }
            })
            .min()
            .unwrap();
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}
fn main() {
    println!("Hello, world!");
    let test_cases = vec![
        vec![8, 6, 1, 5, 3],
        vec![5, 4, 8, 7, 10, 2],
        vec![6, 5, 4, 3, 4, 5],
        vec![48,50,49]
    ];
    for arr in test_cases {
        println!("{}", Solution::minimum_sum(arr))
    }
}
