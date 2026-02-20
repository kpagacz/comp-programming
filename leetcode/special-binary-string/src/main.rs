// https://leetcode.com/problems/special-binary-string/description/?envType=daily-question&envId=2026-02-20
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn make_largest_special(s: String) -> String {
        fn make_special_rec(s: &[u8]) -> Vec<u8> {
            if s.len() == 2 {
                return s.to_owned();
            }

            let mut balance = 0;
            let mut zero_balance = vec![usize::MAX];
            for (i, &c) in s.iter().enumerate() {
                if c == b'1' {
                    balance += 1;
                } else {
                    balance -= 1;
                }
                if balance == 0 {
                    zero_balance.push(i);
                }
            }

            if zero_balance.len() == 2 {
                vec![b'1']
                    .into_iter()
                    .chain(make_special_rec(&s[1..s.len() - 1]))
                    .chain(vec![b'0'])
                    .collect()
            } else {
                let mut sections = zero_balance
                    .windows(2)
                    .map(|window| make_special_rec(&s[window[0].wrapping_add(1)..=window[1]]))
                    .collect::<Vec<_>>();
                sections.sort_by(|a, b| b.cmp(a));

                sections.into_iter().flatten().collect()
            }
        }

        let s = s.as_bytes();
        let answer = make_special_rec(s);
        String::from_utf8(answer).unwrap()
    }
}

fn main() {
    let test_cases = [
        ("11011000", "11100100"),
        ("10", "10"),
        ("101100101100", "110011001010"),
        ("11100010", "11100010"),
    ];
    for (s, exp) in test_cases {
        let s = s.to_owned();
        println!("{} exp: {exp}", Solution::make_largest_special(s));
    }
}
