// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/description/?envType=daily-question&envId=2025-05-03
pub struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        fn check_rotations(target: &[i32], reserve: &[i32], value: i32) -> i32 {
            let mut rots = 0;
            for i in 0..target.len() {
                if target[i] == value {
                    continue;
                } else if reserve[i] == value {
                    rots += 1;
                } else {
                    return i32::MAX;
                }
            }
            rots
        }

        let mut answer = i32::MAX;
        answer = answer.min(check_rotations(&tops, &bottoms, tops[0]));
        answer = answer.min(check_rotations(&tops, &bottoms, bottoms[0]));
        answer = answer.min(check_rotations(&bottoms, &tops, bottoms[0]));
        answer = answer.min(check_rotations(&bottoms, &tops, tops[0]));
        if answer == i32::MAX { -1 } else { answer }
    }
}

fn main() {
    println!("Hello, world!");
}
