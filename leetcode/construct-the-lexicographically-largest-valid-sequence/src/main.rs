// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/description/
pub struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        use std::collections::BTreeSet;
        fn rec(sequence: &mut [i32], mut curr_pos: usize, nums: &mut BTreeSet<i32>) -> bool {
            if curr_pos == sequence.len() {
                return true;
            }
            if sequence[curr_pos] != 0 {
                curr_pos += 1;
                return rec(sequence, curr_pos, nums);
            }
            for &num in nums.clone().iter().rev() {
                let next = curr_pos + num as usize;
                if next < sequence.len() && sequence[next] == 0 {
                    sequence[next] = num;
                    sequence[curr_pos] = num;
                    nums.remove(&num);
                } else {
                    continue;
                }
                if rec(sequence, curr_pos + 1, nums) {
                    return true;
                } else {
                    sequence[next] = 0;
                    sequence[curr_pos] = 0;
                    nums.insert(num);
                }
            }

            if nums.contains(&1) {
                sequence[curr_pos] = 1;
                nums.remove(&1);
                if rec(sequence, curr_pos + 1, nums) {
                    true
                } else {
                    sequence[curr_pos] = 0;
                    nums.insert(1);
                    false
                }
            } else {
                false
            }
        }

        let mut seq = vec![0; 2 * (n - 1) as usize + 1];
        let mut nums = (1..=n).collect::<BTreeSet<i32>>();
        rec(&mut seq, 0, &mut nums);
        seq
    }
}

fn main() {
    let test_cases = [3, 5];

    for n in test_cases {
        println!("{:?}", Solution::construct_distanced_sequence(n));
    }
}
