// https://leetcode.com/problems/add-binary/description/?envType=daily-question&envId=2026-02-15
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = 0;
        let mut result = a
            .as_bytes()
            .iter()
            .rev()
            .chain(std::iter::repeat(&b'0'))
            .zip(b.as_bytes().iter().rev().chain(std::iter::repeat(&b'0')))
            .take(a.len().max(b.len()))
            .map(|(a, b)| {
                let a = *a - b'0';
                let b = *b - b'0';
                let sum = carry + a + b;
                carry = sum / 2;
                (b'0' + (sum % 2)) as char
            })
            .collect::<Vec<_>>();

        if carry > 0 {
            result.push((b'0' + carry) as char);
        }

        result.into_iter().rev().collect::<String>()
    }
}

fn main() {
    let test_cases = [("1", "11", "100")];
    for (a, b, exp) in test_cases {
        let a = a.to_string();
        let b = b.to_string();
        println!("{} exp: {exp}", Solution::add_binary(a, b));
    }
}
