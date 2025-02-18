// https://leetcode.com/problems/construct-smallest-number-from-di-string/description/
pub struct Solution;

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        fn rec(s: &mut Vec<u8>, pattern: &[char], previous: u8) -> Vec<Vec<u8>> {
            if let Some((first, rest)) = pattern.split_first() {
                match first {
                    'I' => {
                        if previous == b'9' {
                            return vec![];
                        }
                        let to_add: Vec<_> =
                            (b'1'..=b'9').filter(|digit| !s.contains(digit)).collect();

                        let mut acc = vec![];
                        for to_add in to_add {
                            if to_add < previous {
                                continue;
                            }
                            s.push(to_add);
                            acc.extend(rec(s, rest, to_add));
                            s.pop();
                        }
                        acc
                    }
                    'D' => {
                        if previous == b'1' {
                            return vec![];
                        }
                        let to_add: Vec<_> =
                            (b'1'..=b'9').filter(|digit| !s.contains(digit)).collect();
                        let mut acc = vec![];
                        for to_add in to_add {
                            if to_add > previous {
                                continue;
                            }
                            s.push(to_add);
                            acc.extend(rec(s, rest, to_add));
                            s.pop();
                        }
                        acc
                    }
                    _ => unreachable!(),
                }
            } else {
                vec![s.clone()]
            }
        }

        let pattern = pattern.chars().collect::<Vec<_>>();
        let mut answers = Vec::default();
        let mut s = Vec::default();
        for start in b'1'..=b'9' {
            s.push(start);
            answers.extend(rec(&mut s, &pattern, start));
            s.pop();
        }
        answers.sort();
        String::from_utf8(answers.first().unwrap().to_vec()).unwrap()
    }
}

fn main() {
    let test_cases = ["IIIDIDDD", "DD", "IID"];
    for pattern in test_cases {
        println!("{:?}", Solution::smallest_number(pattern.to_string()));
    }
}
