// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-ii/description/?envType=daily-question&envId=2026-01-23
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<_> = nums.into_iter().map(|n| n as i64).collect();
        if nums.len() == 1 {
            return 0;
        }

        // Mapping for neighbours
        let mut mapping = vec![(None, None); nums.len()];
        mapping[0] = (None, Some(1));
        mapping
            .iter_mut()
            .enumerate()
            .skip(1)
            .take(nums.len() - 1)
            .for_each(|(pos, map)| {
                map.0 = Some((pos - 1, nums[pos - 1] + nums[pos]));
                map.1 = Some(pos + 1);
            });
        mapping[nums.len() - 1] = (
            Some((
                mapping.len() - 2,
                nums[mapping.len() - 2] + nums[mapping.len() - 1],
            )),
            None,
        );

        use std::collections::BTreeSet;
        use std::collections::HashSet;
        // These track non decreasing pairs - the array
        // is decreasing only if the below set is empty
        // indicating there's no inversions.
        let mut inversions = HashSet::new();
        let mut pair_sums = BTreeSet::new();
        nums.windows(2).enumerate().for_each(|(pos, window)| {
            if window[1] < window[0] {
                inversions.insert((pos, pos + 1));
            }
            pair_sums.insert((window[0] + window[1], pos, pos + 1));
        });

        let mut ops = 0;

        while !inversions.is_empty() {
            // Now it's just maintaing the structures correctly
            let (sum, left_pos, right_pos) = pair_sums
                .pop_first()
                .expect("As lons as there are inversions, there are sums of pairs");

            // Remove the inversion if needed
            inversions.remove(&(left_pos, right_pos));

            // Update nums
            nums[left_pos] = sum;

            // Update left pos mapping
            if let Some((to_the_left, previous_sum)) = mapping[left_pos].0 {
                pair_sums.remove(&(previous_sum, to_the_left, left_pos));
                inversions.remove(&(to_the_left, left_pos));
                mapping[left_pos].0 = Some((to_the_left, nums[to_the_left] + nums[left_pos]));
                pair_sums.insert((nums[to_the_left] + nums[left_pos], to_the_left, left_pos));
                if nums[left_pos] < nums[to_the_left] {
                    inversions.insert((to_the_left, left_pos));
                }
            }
            // Update mapping to the right of right_pos
            if let Some(to_the_right) = mapping[right_pos].1 {
                mapping[left_pos].1 = Some(to_the_right);
                let to_delete = mapping[to_the_right].0.unwrap();
                // println!("removing: {:?}", (to_delete.1, right_pos, to_the_right));
                pair_sums.remove(&(to_delete.1, right_pos, to_the_right));
                inversions.remove(&(right_pos, to_the_right));
                mapping[to_the_right].0 = Some((left_pos, nums[to_the_right] + nums[left_pos]));
                pair_sums.insert((nums[to_the_right] + nums[left_pos], left_pos, to_the_right));
                if nums[to_the_right] < nums[left_pos] {
                    inversions.insert((left_pos, to_the_right));
                }
            } else {
                mapping[left_pos].1 = None;
            }
            mapping[right_pos].0 = None;
            mapping[right_pos].1 = None;

            ops += 1;
        }

        ops
    }
}

fn main() {
    let test_cases = [
        (vec![5, 2, 3, 1], 2),
        (vec![1, 2, 2], 0),
        (vec![1, 0, -100, 20, 10, -5, -10, 7], 5),
        (
            vec![
                5, 2, 3, 1, 7, 0, 1, 10, 9, 8, 7, 0, 1, 2, 3, 4, 100, 1, 2, 3, 4, 5, 6, 7,
            ],
            22,
        ),
    ];

    for (nums, exp) in test_cases {
        println!("=====");
        println!("Nums: {nums:?}");
        println!("{} exp:{exp}", Solution::minimum_pair_removal(nums));
    }
}
