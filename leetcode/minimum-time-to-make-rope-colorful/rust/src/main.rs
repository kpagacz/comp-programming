// https://leetcode.com/problems/minimum-time-to-make-rope-colorful/
pub struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let colors = colors.as_bytes();
        let mut answer = 0;
        let (mut start, mut end) = (0usize, 0usize);

        while start != colors.len() {
            let current_color = colors[start];
            let mut max_cost = i32::MIN;
            while end != colors.len() && colors[end] == current_color {
                answer += needed_time[end];
                max_cost = max_cost.max(needed_time[end]);
                end += 1;
            }
            answer -= max_cost;
            start = end;
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
