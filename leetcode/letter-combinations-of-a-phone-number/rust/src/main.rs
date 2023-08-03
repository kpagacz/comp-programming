// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let digit_to_char = std::collections::HashMap::from([
            (2_u32, vec!["a", "b", "c"]),
            (3, vec!["d", "e", "f"]),
            (4, vec!["g", "h", "i"]),
            (5, vec!["j", "k", "l"]),
            (6, vec!["m", "n", "o"]),
            (7, vec!["p", "q", "r", "s"]),
            (8, vec!["t", "u", "v"]),
            (9, vec!["w", "x", "y", "z"]),
        ]);

        let mut answer = vec!["".to_owned()];
        for digit in digits.chars() {
            let digit = digit.to_digit(10).unwrap();
            answer = answer
                .iter()
                .flat_map(|string| {
                    digit_to_char
                        .get(&digit)
                        .unwrap()
                        .iter()
                        .map(|letter| format!("{}{}", string, letter))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        }

        answer
    }
}

fn main() {
    let test_cases = vec!["23", "", "2", "239"]
        .iter()
        .map(|&s| s.to_owned())
        .collect::<Vec<_>>();
    for test in test_cases {
        println!("{:?}", Solution::letter_combinations(test));
    }
}
