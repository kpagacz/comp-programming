// https://leetcode.com/problems/minimum-number-of-groups-to-create-a-valid-assignment/
pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn min_groups_for_valid_assignment(nums: Vec<i32>) -> i32 {
        let mut freq = nums.iter().fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });
        let max_group_size = *freq.values().min().unwrap();
        let min_group_size = 1;
        let group_size = (min_group_size..=max_group_size)
            .rev()
            .find(|&size| Solution::is_min_group_size_valid(size, &mut freq))
            .unwrap();
        // println!("{group_size}");
        (min_group_size..=max_group_size).rev().for_each(|size| {
            println!(
                "{size} {}",
                Solution::is_min_group_size_valid(size, &mut freq)
            )
        });
        freq.values()
            .map(|&count| {
                let rest = count % (group_size + 1);
                if rest == 0 {
                    count / (group_size + 1)
                } else if count / (group_size + 1) + 1 >= (group_size + 1) - rest {
                    count / (group_size + 1) + 1
                } else {
                    count / group_size
                }
            })
            .sum()
    }

    fn is_min_group_size_valid(size: i32, freq: &mut HashMap<i32, i32>) -> bool {
        freq.values().all(|&count| {
            let rest = count % size;
            count / size >= rest
        })
    }
}
fn main() {
    let test_cases = vec![vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2,
    ]];
    for nums in test_cases {
        println!("{}", Solution::min_groups_for_valid_assignment(nums));
    }
}
