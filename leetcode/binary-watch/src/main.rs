// https://leetcode.com/problems/binary-watch/description/?envType=daily-question&envId=2026-02-17
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut answer = vec![];
        for hours in 0i32..12 {
            for minutes in 0i32..60 {
                if hours.count_ones() + minutes.count_ones() == turned_on {
                    answer.push(format!("{hours}:{minutes:02}"));
                }
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [1, 9, 2, 3];
    for turned_on in test_cases {
        println!("{:?}", Solution::read_binary_watch(turned_on));
    }
}
