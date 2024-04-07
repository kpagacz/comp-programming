// https://leetcode.com/problems/valid-parenthesis-string/description/
pub struct Solution;

impl Solution {
    pub fn check_valid_string_stupid(s: String) -> bool {
        let mut stack = vec![];
        let mut allowed_mistakes = 0;

        for c in s.chars() {
            match c {
                '(' => {
                    stack.push('(');
                }
                ')' => {
                    if stack.pop().is_none() {
                        if allowed_mistakes > 0 {
                            allowed_mistakes -= 1;
                        } else {
                            return false;
                        }
                    }
                }
                '*' => allowed_mistakes += 1,
                _ => {}
            }
        }

        while stack.pop().is_some() {
            if allowed_mistakes > 0 {
                allowed_mistakes -= 1;
            } else {
                return false;
            }
        }

        allowed_mistakes = 0;
        stack.clear();
        for c in s.chars().rev() {
            match c {
                ')' => {
                    stack.push(')');
                }
                '(' => {
                    if stack.pop().is_none() {
                        if allowed_mistakes > 0 {
                            allowed_mistakes -= 1;
                        } else {
                            return false;
                        }
                    }
                }
                '*' => allowed_mistakes += 1,
                _ => {}
            }
        }

        while stack.pop().is_some() {
            if allowed_mistakes > 0 {
                allowed_mistakes -= 1;
            } else {
                return false;
            }
        }

        true
    }

    pub fn check_valid_string(s: String) -> bool {
        let (mut open_lpars, mut expected_rpars) = (0, 0);

        for c in s.chars() {
            match c {
                '(' => {
                    open_lpars += 1;
                    expected_rpars += 1;
                }
                ')' => {
                    if open_lpars > 0 {
                        open_lpars -= 1;
                    }
                    expected_rpars -= 1;
                }
                _ => {
                    if open_lpars > 0 {
                        open_lpars -= 1;
                    }
                    expected_rpars += 1;
                }
            }

            if expected_rpars < 0 {
                return false;
            }
        }

        open_lpars == 0
    }
}

fn main() {
    let test_cases = [
        "()",
        "(*)",
        "(*))",
        "((",
        "(",
        "(*",
        "(((((*",
        "(((((*(((((*((**(((*)*((((**))*)*)))))))))((*(((((**(**)",
        "((*(((((**(**)",
        "((((()((((()(())((()))((((**))*)*)))))))))((*(((((**(**)",
        "((((()(()()()*()(((((*)()*(**(())))))(())()())(((())())())))))))(((((())*)))()))(()((*()*(*)))(*)()"
    ];

    for s in test_cases {
        println!("Raw: {s}");
        println!("{}", Solution::check_valid_string(s.to_owned()));
    }
}
