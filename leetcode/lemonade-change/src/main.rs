// https://leetcode.com/problems/lemonade-change/description/
pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut notes = [0; 2];

        for bill in bills {
            match bill {
                5 => notes[0] += 1,
                10 => {
                    notes[1] += 1;
                    if notes[0] == 0 {
                        return false;
                    } else {
                        notes[0] -= 1;
                    }
                }
                _ => {
                    if notes[1] >= 1 && notes[0] >= 1 {
                        notes[1] -= 1;
                        notes[0] -= 1;
                    } else if notes[0] >= 3 {
                        notes[0] -= 3;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}
