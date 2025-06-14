// https://leetcode.com/problems/fizz-buzz/description/
pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = vec![];
        for i in 1..=n {
            let mut el = String::new();
            if i % 3 == 0 {
                el.push_str("Fizz");
            }
            if i % 5 == 0 {
                el.push_str("Buzz");
            }
            if (i % 3 != 0) && (i % 5 != 0) {
                el.push_str(i.to_string().as_str());
            }
            answer.push(el);
        }
        answer
    }
}

fn main() {
    let test_cases = [15];
    for n in test_cases {
        println!("{:?}", Solution::fizz_buzz(n));
    }
}
