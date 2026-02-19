// https://leetcode.com/problems/trionic-array-i/description/?envType=daily-question&envId=2026-02-03
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let mut increasing = vec![false; nums.len()];
        increasing[0] = true;
        let mut decreasing = vec![false; nums.len()];
        decreasing[0] = true;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                increasing[i] = true;
            } else if nums[i] < nums[i - 1] {
                decreasing[i] = true;
            }
        }

        enum State {
            ConsumeFirstIncreasing,
            ConsumeDecreasing,
            ConsumeSecondIncreasing,
            End,
        }

        let mut state = State::ConsumeFirstIncreasing;
        let mut it = 1;

        while it < nums.len() {
            match (increasing[it], decreasing[it]) {
                (true, false) => {
                    if let State::ConsumeDecreasing = state {
                        state = State::ConsumeSecondIncreasing;
                    }
                }
                (false, true) => {
                    if it == 1 {
                        break;
                    }
                    match state {
                        State::ConsumeFirstIncreasing => state = State::ConsumeDecreasing,
                        State::ConsumeSecondIncreasing => break,
                        _ => {}
                    }
                }
                (false, false) => break,
                _ => unreachable!(),
            }
            it += 1;
            if matches!(state, State::ConsumeSecondIncreasing) && it == nums.len() {
                state = State::End;
            }
        }

        it == nums.len() && matches!(state, State::End)
    }
}

fn main() {
    println!("Hello, world!");
}
