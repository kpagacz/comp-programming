// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/description/
pub struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut opened = vec![];
        let bs = s.as_bytes();
        let mut wormholes = vec![0; s.len()];

        for (pos, c) in bs.iter().enumerate() {
            match c {
                b'(' => opened.push(pos),
                b')' => {
                    let opening_pos = opened.pop().unwrap();
                    wormholes[opening_pos] = pos;
                    wormholes[pos] = opening_pos;
                }
                _ => {}
            }
        }

        let mut answer = String::new();

        let mut it = 0;
        let mut direction = 1i32;

        while it < s.len() {
            match bs[it] {
                b'(' | b')' => {
                    it = wormholes[it];
                    direction = -direction;
                }
                _ => answer.push(bs[it] as char),
            }
            it = (it as i32 + direction) as usize;
        }

        answer
    }
}

fn main() {
    let test_cases = ["(ed(et(oc))el)", "ta()usw((((a))))"];
    for s in test_cases {
        println!("{}", Solution::reverse_parentheses(s.to_string()));
    }
}
