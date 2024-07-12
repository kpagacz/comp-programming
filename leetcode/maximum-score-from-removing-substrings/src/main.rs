// https://leetcode.com/problems/maximum-score-from-removing-substrings/description/
pub struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut stack = vec![];

        let mut answer = 0;

        let removed_first = if x > y { ['a', 'b'] } else { ['b', 'a'] };
        let first_points = x.max(y);

        for c in s.chars() {
            stack.push(c);
            while stack.len() >= 2 && stack[stack.len() - 2..] == removed_first {
                stack.pop();
                stack.pop();
                answer += first_points;
            }
        }

        let mut second_stack = vec![];
        let removed_second = if x > y { ['b', 'a'] } else { ['a', 'b'] };
        let second_points = x.min(y);

        for c in stack {
            second_stack.push(c);
            while second_stack.len() >= 2
                && second_stack[second_stack.len() - 2..] == removed_second
            {
                second_stack.pop();
                second_stack.pop();
                answer += second_points;
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
