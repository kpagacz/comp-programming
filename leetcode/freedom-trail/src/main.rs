// https://leetcode.com/problems/freedom-trail/description/
pub struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.as_bytes();
        let key = key.as_bytes();

        let mut positions = vec![Vec::new(); 256];
        for (pos, c) in ring.iter().enumerate() {
            positions[*c as usize].push(pos);
        }

        let mut dp = vec![i32::MAX; ring.len()];
        dp[0] = 0;

        for &c in key {
            let mut new_dp = vec![i32::MAX; ring.len()];
            for (previous_pos, &rotations) in dp.iter().enumerate() {
                if rotations != i32::MAX {
                    for position in positions[c as usize].iter() {
                        let additional_rotations = Solution::calculate_additional_rotations(
                            previous_pos,
                            *position,
                            ring.len(),
                        );
                        new_dp[*position] = new_dp[*position].min(rotations + additional_rotations);
                    }
                }
            }

            dp = new_dp;
        }

        *dp.iter().min().unwrap() as i32 + key.len() as i32
    }

    fn calculate_additional_rotations(start: usize, end: usize, array: usize) -> i32 {
        let start = start as i32;
        let end = end as i32;
        let array = array as i32;
        if start == end {
            return 0;
        }
        if start > end {
            (start - end).min(array - start + end)
        } else {
            (end - start).min(array - end + start)
        }
    }
}

fn main() {
    let test_cases = vec![("godding", "gd"), ("godding", "godding"), ("g", "g")];

    for (ring, key) in test_cases {
        println!(
            "{}",
            Solution::find_rotate_steps(ring.to_string(), key.to_string())
        );
    }
}
