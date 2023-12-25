// https://leetcode.com/problems/decode-ways/description/?envType=daily-question&envId=2023-12-25
pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let allowed_after_two = ['0', '1', '2', '3', '4', '5', '6'];
        let mut dp = (1, 0, 0);
        for i in (0..s.len()).rev() {
            dp.2 = dp.1;
            dp.1 = dp.0;
            dp.0 = 0;
            if s[i] == '0' {
                dp.0 = 0;
            } else {
                dp.0 += dp.1;
                if i < s.len() - 1
                    && (s[i] == '1' || (s[i] == '2' && allowed_after_two.contains(&s[i + 1])))
                {
                    dp.0 += dp.2;
                }
            }
        }
        dp.0
    }
}

fn main() {
    let test_cases = [
        "12",
        "226",
        "06",
        "1234567810123456789123456789123456789123456789123456789123456781012345678101234567810",
        "0",
        "1",
    ];
    for test in test_cases {
        println!("{} {}", test, Solution::num_decodings(test.to_string()));
    }
}
