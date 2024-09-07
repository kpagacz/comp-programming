// https://leetcode.com/problems/find-the-key-of-the-numbers/description/
pub struct Solution;

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let num1 = format!("{:0>4}", num1);
        let num2 = format!("{:0>4}", num2);
        let num3 = format!("{:0>4}", num3);
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let num3 = num3.as_bytes();

        let mut key = [u8::MAX; 4];

        key.iter_mut().enumerate().for_each(|(pos, digit)| {
            for &num in &[num1, num2, num3] {
                *digit = (*digit).min(num[pos]);
            }
        });

        std::str::from_utf8(&key).unwrap().parse::<i32>().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
