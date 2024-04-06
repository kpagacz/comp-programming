// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/description/
pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        type ParPos = (char, usize);
        let mut stack: Vec<ParPos> = Vec::new();
        use std::collections::HashSet;
        let mut to_remove = HashSet::new();

        s.chars().enumerate().for_each(|(id, c)| match c {
            '(' => {
                stack.push((c, id));
            }
            ')' => match stack.last() {
                Some((c, _)) if *c == '(' => {
                    stack.pop();
                }
                _ => {
                    to_remove.insert(id);
                }
            },
            _ => {}
        });

        while let Some((_, id)) = stack.pop() {
            to_remove.insert(id);
        }

        String::from_iter(s.chars().enumerate().filter_map(|(id, c)| {
            if !to_remove.contains(&id) {
                Some(c)
            } else {
                None
            }
        }))
    }
}

fn main() {
    let tests = ["lee(t(c)o)de)", "a)b(c)d", "))((", "(()", "((("];
    for test in tests {
        println!("Raw: {test}");
        println!("{}", Solution::min_remove_to_make_valid(test.to_owned()));
    }
}
