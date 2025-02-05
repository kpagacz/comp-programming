// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/description/
pub struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }

        let errors = s1
            .as_bytes()
            .iter()
            .zip(s2.as_bytes())
            .fold(
                (0, 0u8, 0u8, 0),
                |(total_mistakes, previous1, previous2, step), (c1, c2)| {
                    if c1 == c2 {
                        (total_mistakes, previous1, previous2, step)
                    } else {
                        match step {
                            0 => (total_mistakes + 1, *c1, *c2, 1),
                            1 => {
                                if previous1 == *c2 && previous2 == *c1 {
                                    (total_mistakes + 1, *c1, *c2, 2)
                                } else {
                                    (total_mistakes + 3, *c1, *c2, 3)
                                }
                            }
                            _ => (total_mistakes + 3, *c1, *c2, 3),
                        }
                    }
                },
            )
            .0;
        errors == 2 || errors == 0
    }
}

fn main() {
    println!("Hello, world!");
}
