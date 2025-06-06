// https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/description/?envType=daily-question&envId=2025-06-06
pub struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut counts = [0; 26];
        for &c in s.as_bytes() {
            counts[(c - b'a') as usize] += 1;
        }
        let mut answer = String::with_capacity(s.len());
        let mut stack = vec![];
        for c in s.chars() {
            stack.push(c);
            let offset = (c as u8 - b'a') as usize;
            counts[offset] -= 1;

            let first_non_zero = counts
                .into_iter()
                .enumerate()
                .filter_map(|(offset, count)| {
                    if count == 0 {
                        None
                    } else {
                        Some(b'a' + offset as u8)
                    }
                })
                .next()
                .unwrap_or(b'z');
            while !stack.is_empty() && (*stack.last().unwrap() as u8) < first_non_zero {
                answer.push(stack.pop().unwrap());
            }
        }

        while let Some(c) = stack.pop() {
            answer.push(c);
        }
        answer
    }
}

fn main() {
    let test_cases = [
        ("zza", "azz"),
        ("bac", "abc"),
        ("bdda", "addb"),
        ("bydizfve", "bdevfziy"),
    ];
    for (test, exp) in test_cases {
        println!("test: {test}");
        println!(
            "result: {}, exp: {exp}",
            Solution::robot_with_string(test.to_string())
        );
    }
}
