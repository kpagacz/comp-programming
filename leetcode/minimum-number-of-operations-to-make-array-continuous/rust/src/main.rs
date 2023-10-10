// https://leetcode.com/problems/minimum-number-of-operations-to-make-array-continuous/description
pub struct Solution {}
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let prefix_duplicates = (0..1)
            .chain(nums.windows(2).scan(0, |sum, adj| {
                if adj[0] == adj[1] {
                    *sum += 1;
                    Some(*sum)
                } else {
                    Some(*sum)
                }
            }))
            .collect::<Vec<i32>>();
        // println!("{prefix_duplicates:?}");
        (0..nums.len())
            .map(|id| {
                let after_last = nums.partition_point(|&el| el <= nums[id] + nums.len() as i32 - 1);
                nums.len() as i32 - (after_last - id) as i32 + prefix_duplicates[after_last - 1]
                    - prefix_duplicates[id]
            })
            .min()
            .unwrap()
    }

    pub fn min_operations2(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        nums.sort_unstable();
        nums.dedup();

        nums.iter().enumerate().fold(n, |min_ops, (id, el)| {
            min_ops.min(n - (nums.partition_point(|&m| m < el + n) - id) as i32)
        })
    }
}
fn main() {
    println!("Hello, world!");
}
